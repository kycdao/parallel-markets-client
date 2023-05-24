mod error;
mod types;

pub use error::*;
use json_api_client::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
pub use types::*;

pub use json_api_client::{AccessToken, AuthorizationCode, RefreshToken, StandardToken, Token};

pub struct Client {
    api: ApiClient,
    scopes: Vec<Scope>,
    //client_id: String,
    //client_secret: String,
}

impl Client {
    pub fn new(api_url: &str, client_id: &str, client_secret: &str, redirect_url: &str, scopes: &[Scope]) -> Result<Client> {
        let scopes_str: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
        let conf = OAuth2Config {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            authorize_path: "oauth/authorize".to_string(),
            auth_type: AuthType::RequestBody,
            token_path: "oauth/token".to_string(),
            refresh_path: "oauth/refresh".to_string(),
            redirect_url: redirect_url.to_string(),
            scopes: scopes_str,
        };

        let client = ApiClient::new(api_url, AuthConfig::OAuth2(conf), None)?;

        Ok(Client {
            api: client,
            scopes: scopes.to_vec(),
            //client_id: client_id.to_owned(),
            //client_secret: client_secret.to_owned(),
        })
    }

    fn oauth_header(token: &str) -> HeaderMap {
        let auth_header = format!("Bearer {}", token);
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&auth_header).expect("Invalid API token value");
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);
        headers
    }

    async fn get<T>(&self, path: &str, token: &str) -> Result<T>
    where
        T: JsonResponse,
    {
        let header = Client::oauth_header(token);
        self.api.get(path, None, Some(header)).await.map_err(Error::from)
    }

    pub async fn exchange_code(&self, code: AuthorizationCode) -> Result<StandardToken> {
        self.api.exchange_code(code).await.map_err(Error::from)
    }

    pub async fn refresh_token(&self, token: &RefreshToken) -> Result<StandardToken> {
        self.api.refresh(token).await.map_err(Error::from)
    }

    fn ensure_scope(&self, scope: Scope) -> Result<()> {
        if !self.scopes.contains(&scope) {
            return Err(Error::ApiError(ErrorKind::ScopeNotEnabled(scope)));
        }
        Ok(())
    }

    pub async fn get_profile(&self, token: &str) -> Result<ProfileResponse> {
        self.ensure_scope(Scope::Profile)?;
        self.get("me", token).await
    }

    pub async fn get_accreditations(&self, token: &str) -> Result<AccreditationsResponse> {
        self.ensure_scope(Scope::AccreditationStatus)?;
        self.get("accreditations", token).await
    }

    pub async fn get_identity(&self, token: &str) -> Result<IdentityResponse> {
        self.ensure_scope(Scope::Identity)?;
        self.get("identity", token).await
    }

    /// dependency_id: ID from ControlPersonReference or BeneficialOwnerReference
    pub async fn get_dependency_identity(&self, dependency_id: &str, token: &str) -> Result<DependencyIdentityResponse> {
        let path = format!("identity/{}", dependency_id);
        self.get(&path, token).await
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // put unittests here
}
