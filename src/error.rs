use crate::Scope;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ApiError(#[from] ErrorKind),

    #[error(transparent)]
    ClientError(#[from] json_api_client::error::Error),
}

// TODO KYC-136 add more variants for expected API errors
#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Scope not enabled: '{0}'")]
    ScopeNotEnabled(Scope),
}

pub type Result<T> = std::result::Result<T, Error>;
