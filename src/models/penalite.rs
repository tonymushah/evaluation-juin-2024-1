use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, schema::penalite};

use super::{equipe::Equipe, etape::Etape};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Identifiable,
    Selectable,
    Insertable,
    Queryable,
    AsChangeset,
    SimpleObject,
    InputObject,
)]
#[graphql(input_name = "PenaliteInput")]
#[graphql(complex)]
#[diesel(table_name = penalite)]
#[diesel(primary_key(id_penalite))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Penalite {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_penalite: Uuid,
    pub etape: i32,
    pub valeur: i32,
    pub equipe: Uuid,
}

#[ComplexObject]
impl Penalite {
    pub async fn equipe_data(&self, ctx: &Context<'_>) -> crate::Result<Equipe> {
        let id = self.equipe;
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(Equipe::as_select())
                .filter(id_equipe.eq(id))
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn etape_data(&self, ctx: &Context<'_>) -> crate::Result<Etape> {
        let id = self.etape;
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(etape
                .select(Etape::as_select())
                .filter(rang.eq(id))
                .get_result(&mut pool)?)
        })
        .await
    }
}
