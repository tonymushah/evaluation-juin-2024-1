use async_graphql::{ComplexObject, Context, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, view::v_equipe_point};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[graphql(complex)]
#[diesel(table_name = v_equipe_point)]
#[diesel(primary_key(equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipePoint {
    pub equipe: Uuid,
    pub points: i64,
    pub temps: i64,
}

#[ComplexObject]
impl EquipePoint {
    pub async fn nom(&self, ctx: &Context<'_>) -> crate::Result<String> {
        let equ = self.equipe;
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(nom)
                .filter(id_equipe.eq(equ))
                .get_result::<String>(&mut pool)?)
        })
        .await
    }
}
