pub mod unique;

use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{etape::Etape, Paginate},
};

use self::unique::AdminEtape;

#[derive(Clone, Copy, Debug, Default)]
pub struct EtapeQueries;

#[Object]
impl EtapeQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<AdminEtape>> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(etape
                .select(Etape::as_select())
                .paginate_with_param(pagination)
                .to_results_data::<Etape>(&mut pool)?
                .map_into())
        })
        .await
    }
    pub async fn unique(&self, ctx: &Context<'_>, rang: i32) -> crate::Result<AdminEtape> {
        let id = rang;
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(etape
                .filter(rang.eq(id))
                .select(Etape::as_select())
                .get_result(&mut pool)?
                .into())
        })
        .await
    }
}
