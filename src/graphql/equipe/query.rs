pub mod etapes;

use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{equipe_coureur::VEquipeCoureur, Paginate},
};

use self::etapes::EtapeQueries;
use super::token::ExtractCurrentEquipe;

#[derive(Debug, Clone, Copy, Default)]
pub struct EquipeQueries;

#[Object]
impl EquipeQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn etape(&self) -> EtapeQueries {
        EtapeQueries
    }
    pub async fn list_coureur(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<VEquipeCoureur>> {
        let current = ctx.get_current_equipe()?;
        ctx.use_pool(move |mut pool| {
            use crate::view::v_equipe_coureur::dsl::*;
            Ok(v_equipe_coureur
                .select(VEquipeCoureur::as_select())
                .filter(equipe.eq(current.0))
                .paginate_with_param(pagination)
                .to_results_data(&mut pool)?)
        })
        .await
    }
    pub async fn coureur(&self, ctx: &Context<'_>, dosard: i32) -> crate::Result<VEquipeCoureur> {
        let current = ctx.get_current_equipe()?;
        ctx.use_pool(move |mut pool| {
            use crate::view::v_equipe_coureur::dsl::*;
            Ok(v_equipe_coureur
                .select(VEquipeCoureur::as_select())
                .filter(equipe.eq(current.0))
                .filter(coureur.eq(dosard))
                .get_result(&mut pool)?)
        })
        .await
    }
}
