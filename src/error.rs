use std::num::ParseIntError;

use async_graphql::ErrorExtensions;
use diesel::r2d2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Pool(#[from] r2d2::Error),
    #[error("{0}")]
    PoolConnection(#[from] r2d2::PoolError),
    #[error("{0}")]
    DieselConnection(#[from] diesel::ConnectionError),
    #[error("{0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("The env variable {0} is not found")]
    EnvNotFound(String),
    #[error("{0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("an graphql error occured")]
    GraphQL(async_graphql::Error),
    #[error("{0}")]
    Blocking(#[from] actix_web::error::BlockingError),
    #[error("{0}")]
    JWT(#[from] jwt::Error),
    #[error("You cannot access this ressource")]
    Forbidden,
    #[error("The upsert result is not found ")]
    UpsertNotFound,
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("can not find the capture name {0}")]
    RegexCaptureNameNotFound(String),
    #[error("An unexpected `std::num::TryFromIntError` was caught")]
    TryFromInt,
}

impl From<async_graphql::Error> for Error {
    fn from(value: async_graphql::Error) -> Self {
        Self::GraphQL(value)
    }
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        if let Self::GraphQL(error) = self {
            error.clone()
        } else {
            async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
                Error::Pool(_) => e.set("code", "DB_POOL"),
                Error::DieselConnection(_) => e.set("code", "DB_CONNECT"),
                Error::Diesel(er) => match er {
                    diesel::result::Error::InvalidCString(_) => e.set("code", "INVALID_CS_STRING"),
                    diesel::result::Error::DatabaseError(_, _) => e.set("code", "DB_INNER"),
                    diesel::result::Error::NotFound => e.set("code", "NOT_FOUND"),
                    diesel::result::Error::QueryBuilderError(_) => e.set("code", "QUERY_BUILDER"),
                    diesel::result::Error::DeserializationError(_) => {
                        e.set("code", "DESERIALIZATION")
                    }
                    diesel::result::Error::SerializationError(_) => e.set("code", "SERIALIZATION"),
                    diesel::result::Error::RollbackErrorOnCommit {
                        rollback_error,
                        commit_error,
                    } => {
                        e.set("code", "ROLLBACK_ON_COMMIT");
                        e.set("rollback", rollback_error.to_string());
                        e.set("commit", commit_error.to_string())
                    }
                    diesel::result::Error::RollbackTransaction => {
                        e.set("code", "ROLLBACK_TRANSACTION")
                    }
                    diesel::result::Error::AlreadyInTransaction => {
                        e.set("code", "ALREADY_IN_TRANSACTION")
                    }
                    diesel::result::Error::NotInTransaction => e.set("code", "NOT_IN_TRANSACTION"),
                    diesel::result::Error::BrokenTransactionManager => {
                        e.set("code", "BROKEN_TRANSACTION_MANAGER")
                    }
                    _ => e.set("code", "UNKNOW_DIESEL_ERROR"),
                },
                Error::EnvNotFound(name) => {
                    e.set("code", "ENV_NOT_FOUND");
                    e.set("env_name", name);
                }
                Error::EnvVar(er) => match er {
                    std::env::VarError::NotPresent => e.set("code", "ENV_NOT_FOUND"),
                    std::env::VarError::NotUnicode(_) => e.set("code", "NOT_UNICODE"),
                },
                Error::PoolConnection(_) => e.set("code", "POOL_CONNECTION"),
                Error::GraphQL(_) => e.set("code", "INNER_GRAPHQL"),
                Error::Blocking(_) => e.set("code", "TASK_BLOCKING"),
                Error::JWT(er) => match er {
                    jwt::Error::AlgorithmMismatch(_, _) => e.set("code", "ALGORITHM_MISMATCH"),
                    jwt::Error::Base64(_) => e.set("code", "BASE64"),
                    jwt::Error::Format => e.set("code", "JWT_FORMAT"),
                    jwt::Error::InvalidSignature => e.set("code", "INVALID_SIGNATURE"),
                    jwt::Error::Json(_) => e.set("code", "JWT_JSON"),
                    jwt::Error::NoClaimsComponent => e.set("code", "NO_CLAIMS_COMPONENT"),
                    jwt::Error::NoHeaderComponent => e.set("code", "NO_HEADER_COMPONENT"),
                    jwt::Error::NoKeyId => e.set("code", "NO_KEY_ID"),
                    jwt::Error::NoKeyWithKeyId(_) => e.set("code", "NO_KEY_WITH_KEY_ID"),
                    jwt::Error::NoSignatureComponent => e.set("code", "NO_SIGNATURE_COMPONENT"),
                    jwt::Error::RustCryptoMac(_) => e.set("code", "RUST_CRYPTO_MAC"),
                    jwt::Error::RustCryptoMacKeyLength(_) => {
                        e.set("code", "RUST_CRYPTO_MAC_KEY_LENGTH")
                    }
                    jwt::Error::TooManyComponents => e.set("code", "TOO_MANY_COMPONENTS"),
                    jwt::Error::Utf8(_) => e.set("code", "UTF-8"),
                },
                Error::Forbidden => e.set("code", "FORBIDDEN"),
                Error::UpsertNotFound => e.set("code", "UPSERT_NOT_FOUND"),
                Error::Regex(_) => e.set("code", "REGEX"),
                Error::ParseInt(_) => e.set("code", "PARSE_INT"),
                Error::RegexCaptureNameNotFound(_) => e.set("code", "REGEX_CAPTURE_NAME_NOT_FOUND"),
                Error::TryFromInt => e.set("code", "TRY_FROM_INT"),
            })
        }
    }
}
