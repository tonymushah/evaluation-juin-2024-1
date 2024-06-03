pub mod etape;

use self::etape::EtapeMutation;

use async_graphql::{Context, Object};

use crate::{graphql::GetPoolConnection, reset::reset_db};

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
