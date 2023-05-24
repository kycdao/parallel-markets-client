use parallel_markets_client::*;
use test_log::test;

const STAGING_URL: &str = "https://demo-api.parallelmarkets.com/v1/";
const CLIENT_ID: &str = "REPLACE_ME";
const CLIENT_SECRET: &str = "REPLACE_ME";
const REDIRECT_URL: &str = "https://kycdao.xyz/test.html";

// Note: Put your own tokens here! - beware, these tokens expire, so they probably have to be replaced regularly
const ACCESS_TOKEN: &str = "REPLACE_ME";
const REFRESH_TOKEN: &str = "REPLACE_ME";

fn get_client() -> Client {
    let scopes = vec![Scope::Profile, Scope::Identity, Scope::AccreditationStatus];
    Client::new(STAGING_URL, CLIENT_ID, CLIENT_SECRET, REDIRECT_URL, &scopes).unwrap()
}

#[test(tokio::test)]
#[ignore]
async fn exchange_code() {
    let client = get_client();
    let code = AuthorizationCode::from("TEST".to_string());
    let resp = client.exchange_code(code).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn refresh_token() {
    let client = get_client();
    let token = RefreshToken::from(REFRESH_TOKEN);
    let resp = client.refresh_token(&token).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
async fn get_profile() {
    let client = get_client();
    let resp = client.get_profile(ACCESS_TOKEN).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
async fn get_accreditations() {
    let client = get_client();
    let resp = client.get_accreditations(ACCESS_TOKEN).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_identity() {
    let client = get_client();
    let resp = client.get_identity(ACCESS_TOKEN).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_dependency_identity() {
    let client = get_client();
    let dependency_id = "VXNlcjoyODIw";
    let resp = client.get_dependency_identity(dependency_id, ACCESS_TOKEN).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}
