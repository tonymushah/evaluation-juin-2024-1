use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{penalite::Penalite, Paginate},
};

pub struct PenalitesQueries;

#[Object]
impl PenalitesQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<Penalite>> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::penalite::dsl::*;
            Ok(penalite
                .select(Penalite::as_select())
                .paginate_with_param(pagination)
                .to_results_data(&mut pool)?)
        })
        .await
    }
}
