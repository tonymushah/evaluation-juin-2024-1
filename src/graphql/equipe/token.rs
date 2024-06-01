use std::{env, ops::Deref};

use dotenvy::dotenv;
use hmac::{digest::KeyInit, Hmac};
use sha2::Sha256;

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
