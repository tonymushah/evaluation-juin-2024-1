pub mod equipe;
pub mod etape;
pub mod penalites;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{categorie::Categorie, Paginate},
};

use self::{equipe::EquipeQueries, etape::EtapeQueries, penalites::PenalitesQueries};

use async_graphql::{Context, Object};
use diesel::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct AdminQueries;

#[Object]
impl AdminQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn etape(&self) -> EtapeQueries {
        EtapeQueries
    }
    pub async fn penalite(&self) -> PenalitesQueries {
        PenalitesQueries
    }
    pub async fn equipe(&self) -> EquipeQueries {
        EquipeQueries
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
