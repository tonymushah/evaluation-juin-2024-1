use std::ops::Deref;

use async_graphql::{Context, Object};
use diesel::prelude::*;
use jwt::SignWithKey;
use uuid::Uuid;

use crate::graphql::GetPoolConnection;

use super::token::ClientHmac;

#[derive(Debug, Clone, Default, Copy)]
pub struct EquipeMutations;

#[Object]
impl EquipeMutations {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> crate::Result<String> {
        let sha = ctx.data::<ClientHmac>()?.clone();
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(id_equipe)
                .filter(pseudo.eq(username))
                .filter(mot_passe.eq(password))
                .get_result::<Uuid>(&mut pool)?
                .sign_with_key(sha.deref())?)
        })
        .await
    }
}
