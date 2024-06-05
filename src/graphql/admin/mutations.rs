pub mod etape;
pub mod import;
pub mod penalites;

use self::{etape::EtapeMutation, import::ImportMutations, penalites::PenaliteMuations};

use std::ops::Deref;

use async_graphql::{Context, Object};
use jwt::SignWithKey;
use uuid::Uuid;

use crate::{
    graphql::GetPoolConnection, modules::generate_categories::SystemCategories, reset::reset_db,
};

use super::token::{extract_admin_password, AdminHmac, VerifyAdminSession};

#[derive(Debug, Clone, Copy, Default)]
pub struct AdminMutations;

#[Object]
impl AdminMutations {
    pub async fn reset_db(&self, ctx: &Context<'_>) -> crate::Result<bool> {
        ctx.use_pool(|mut pool| {
            reset_db(&mut pool)?;
            Ok(())
        })
        .await?;
        Ok(true)
    }
    pub async fn etape(&self, id: i32) -> EtapeMutation {
        EtapeMutation(id)
    }
    pub async fn login(&self, ctx: &Context<'_>, password: String) -> crate::Result<String> {
        let pass = extract_admin_password()?;
        let sha = ctx.data::<AdminHmac>()?;
        if pass == password {
            let secret = Uuid::new_v4().sign_with_key(sha.deref())?;
            ctx.update(secret.clone()).await?;
            Ok(secret)
        } else {
            Err(crate::Error::Forbidden)
        }
    }
    pub async fn import(&self) -> ImportMutations {
        ImportMutations
    }
    pub async fn logout(&self, ctx: &Context<'_>) -> crate::Result<bool> {
        ctx.reset().await?;
        Ok(true)
    }
    pub async fn penalite(&self) -> PenaliteMuations {
        PenaliteMuations
    }
    pub async fn generate_categories(&self, ctx: &Context<'_>) -> crate::Result<usize> {
        ctx.use_pool_transaction(|pool| {
            let _ = SystemCategories::seed(pool);
            Ok(SystemCategories::generate_categories(pool)?.len())
        })
        .await
    }
}

#[macro_export]
macro_rules! generate_crud_mutation {
    {
        #[$mutation:ident] $parent:ident,
        #[$base:ident] $base_:ty,
        #[$dsl:ident] $dsl_:path,
        #[$table:ident] $table_:expr,
        #[$id:ident] $id_:expr => $id_type:ty,
        #[$input:ident] $input_:ty
    } => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $parent;
        #[async_graphql::Object]
        impl $parent {
            pub async fn upsert(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: $input_,
            ) -> $crate::Result<$base_> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<$base_> {
                    use $dsl_::*;
                    let to_input: $base_ = input.into();
                    diesel::insert_into($table_)
                        .values(&to_input)
                        .on_conflict($id_)
                        .do_update()
                        .set(&to_input)
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?
                        .first()
                        .cloned()
                        .ok_or($crate::Error::UpsertNotFound)
                })
                .await?
            }
            pub async fn upsert_batch(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: Vec<$input_>,
            ) -> $crate::Result<Vec<$base_>> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<Vec<$base_>> {
                    use $dsl_::*;
                    let to_input: Vec<$base_> = input.into_iter().map(|i| i.into()).collect();
                    let mut res = diesel::insert_into($table_)
                        .values(&to_input)
                        .on_conflict($id_)
                        .do_nothing()
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?;
                    for i in &to_input {
                        res.append(&mut diesel::update($table_).set(i).returning(<$base_>::as_returning()).get_results(&mut pool)?);
                    }
                    Ok(res)
                })
                .await?
            }
            pub async fn delete(
                &self,
                ctx: &async_graphql::Context<'_>,
                id: $id_type,
            ) -> $crate::Result<$base_> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<$base_> {
                    use $dsl_::*;
                    Ok(diesel::delete($table_.filter($id_.eq(id)))
                        .returning(<$base_>::as_returning())
                        .get_result(&mut pool)?)
                })
                .await?
            }
            pub async fn delete_batch(
                &self,
                ctx: &async_graphql::Context<'_>,
                ids: Vec<$id_type>,
            ) -> $crate::Result<Vec<$base_>> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<Vec<$base_>> {
                    use $dsl_::*;
                    let res = diesel::delete($table_.filter($id_.eq_any(&ids)))
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?;
                    Ok(res)
                })
                .await?
            }
        }
    }
}
