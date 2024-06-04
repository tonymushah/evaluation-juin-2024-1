use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{equipe::Equipe, Paginate},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct EquipeQueries;

#[Object]
impl EquipeQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<Equipe>> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(Equipe::as_select())
                .paginate_with_param(pagination)
                .to_results_data::<Equipe>(&mut pool)?
                .map_into())
        })
        .await
    }
}
