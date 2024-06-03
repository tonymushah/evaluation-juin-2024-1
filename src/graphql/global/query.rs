pub mod classement;

use async_graphql::{Context, Object};
use diesel::prelude::*;

use self::classement::ClassementQueries;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{categorie::Categorie, Paginate},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalQueries;

#[Object]
impl GlobalQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn classements(&self) -> ClassementQueries {
        ClassementQueries
    }
    pub async fn categories(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<Categorie>> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::categorie::dsl::*;
            Ok(categorie
                .select(Categorie::as_select())
                .paginate_with_param(pagination)
                .to_results_data(&mut pool)?)
        })
        .await
    }
}
