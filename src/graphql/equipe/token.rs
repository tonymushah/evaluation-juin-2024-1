use std::{env, ops::Deref};

use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use async_graphql::Context;
use dotenvy::dotenv;
use hmac::{digest::KeyInit, Hmac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClientHmac(Hmac<Sha256>);

impl Deref for ClientHmac {
    type Target = Hmac<Sha256>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ClientHmac {
    pub fn extract() -> ClientHmac {
        dotenv().ok();
        let client_hmac =
            env::var("CLIENTHMAC").expect("Cannot find the `CLIENTHMAC` env variable");
        Self(Hmac::new_from_slice(client_hmac.as_bytes()).unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentEquipe(pub Uuid);

pub trait ExtractCurrentEquipe {
    fn get_current_equipe(&self) -> crate::Result<CurrentEquipe>;
}

impl<'a> ExtractCurrentEquipe for Context<'a> {
    fn get_current_equipe(&self) -> crate::Result<CurrentEquipe> {
        let client_hmac = self.data::<ClientHmac>()?;
        let headers = self.data::<HeaderMap>()?;
        let token = headers.get(AUTHORIZATION).ok_or(crate::Error::Forbidden)?;
        let id: Uuid = token.to_str()?.verify_with_key(client_hmac.deref())?;
        Ok(CurrentEquipe(id))
    }
}
