use async_graphql::{Context, Object};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    graphql::{equipe::token::ExtractCurrentEquipe, GetPoolConnection, OffsetLimit, ResultsData},
    models::{equipe_coureur::VEquipeCoureur, etape::Etape, Paginate},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct EtapeQueries;

#[Object]
impl EtapeQueries {
    pub async fn list(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<Etape>> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(etape
                .select(Etape::as_select())
                .paginate_with_param(pagination)
                .to_results_data::<Etape>(&mut pool)?)
        })
        .await
    }
    pub async fn unique(&self, ctx: &Context<'_>, rang_: i32) -> crate::Result<Etape> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(etape
                .select(Etape::as_select())
                .filter(rang.eq(rang_))
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn joueurs(
        &self,
        ctx: &Context<'_>,
        etape_: i32,
    ) -> crate::Result<Vec<VEquipeCoureur>> {
        let current = ctx.get_current_equipe()?;
        ctx.use_pool(move |mut pool| {
            let js: Vec<Uuid> = {
                use crate::view::v_temps_coureur_etape_equipe_coureur::dsl::*;
                v_temps_coureur_etape_equipe_coureur
                    .select(equipe_coureur)
                    .filter(etape.eq(etape_))
                    .filter(equipe.eq(current.0))
                    .get_results(&mut pool)?
            };
            Ok({
                use crate::view::v_equipe_coureur::dsl::*;
                v_equipe_coureur
                    .select(VEquipeCoureur::as_select())
                    .filter(id_equipe_coureur.eq_any(&js))
                    .get_results(&mut pool)?
            })
        })
        .await
    }
}
