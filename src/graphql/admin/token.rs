use std::{
    env,
    future::Future,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use async_graphql::Context;
use dotenvy::dotenv;
use hmac::{digest::KeyInit, Hmac};
use sha2::Sha256;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AdminHmac(Hmac<Sha256>);

impl Deref for AdminHmac {
    type Target = Hmac<Sha256>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AdminHmac {
    pub fn extract() -> AdminHmac {
        dotenv().ok();
        let client_hmac = env::var("ADMINHMAC").expect("Cannot find the `ADMINHMAC` env variable");
        Self(Hmac::new_from_slice(client_hmac.as_bytes()).unwrap())
    }
}

type AdminSessionInner = Arc<RwLock<Option<String>>>;

#[derive(Debug, Clone, Default)]
pub struct AdminSession(AdminSessionInner);

impl Deref for AdminSession {
    type Target = AdminSessionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdminSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait VerifyAdminSession {
    fn is_empty(&self) -> impl Future<Output = bool> + Send;
    fn verify(&self) -> impl Future<Output = bool> + Send;
    fn verify_res(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send
    where
        Self: std::marker::Sync,
    {
        async {
            if self.verify().await {
                Ok(())
            } else {
                Err(crate::Error::Forbidden)
            }
        }
    }
    fn reset(&self) -> impl Future<Output = crate::Result<()>> + Send;
    fn update(&self, secret: String) -> impl Future<Output = crate::Result<()>> + Send;
}

pub fn extract_admin_password() -> crate::Result<String> {
    dotenv().ok();
    Ok(env::var("ADMIN_SECRET")?)
}

impl<'a> VerifyAdminSession for Context<'a> {
    async fn is_empty(&self) -> bool {
        match self.data::<AdminSession>() {
            Ok(session) => session.read().await.is_none(),
            Err(_) => true,
        }
    }

    async fn verify(&self) -> bool {
        let Some(auth) = self.data::<HeaderMap>().ok().and_then(|hs| {
            hs.get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok().map(String::from))
        }) else {
            return false;
        };
        match self.data::<AdminSession>() {
            Ok(session) => {
                let read = session.read().await;
                read.cmp(&Some(auth)).is_eq()
            }
            Err(_) => false,
        }
    }

    async fn reset(&self) -> crate::Result<()> {
        let session = self.data::<AdminSession>()?;
        let mut write = session.write().await;
        write.take();
        Ok(())
    }

    async fn update(&self, secret: String) -> crate::Result<()> {
        let session = self.data::<AdminSession>()?;
        let mut write = session.write().await;
        write.replace(secret);
        Ok(())
    }
}
