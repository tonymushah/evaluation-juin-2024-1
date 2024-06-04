pub mod etapes;

use async_graphql::{Context, Object};
use diesel::{expression::expression_types::NotSelectable, pg::Pg, prelude::*};
use uuid::Uuid;

use crate::{
    graphql::{objects::order::GraphQLOrdering, GetPoolConnection, OffsetLimit, ResultsData},
    models::{
        classement::classement_categorie_equipe::ClassementCategorieEquipe,
        coureur_point::CoueurPoint, equipe_coureur::VEquipeCoureur, Paginate,
    },
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
    pub async fn classement_par_categorie(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
        #[graphql(default)] pagination: OffsetLimit,
        categorie_: Uuid,
    ) -> crate::Result<ResultsData<CoueurPoint>> {
        let current = ctx.get_current_equipe()?;
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_classement_categories_equipe::dsl::*;
            let orde: Box<
                dyn BoxableExpression<v_classement_categories_equipe, Pg, SqlType = NotSelectable>,
            > = match ordre {
                GraphQLOrdering::Ascending => Box::new(points.asc()),
                GraphQLOrdering::Descending => Box::new(points.desc()),
            };
            Ok(v_classement_categories_equipe
                .order(orde)
                .filter(equipe.eq(current.0))
                .filter(categorie.eq(categorie_))
                .select(ClassementCategorieEquipe::as_select())
                .paginate_with_param(pagination)
                .to_results_data::<ClassementCategorieEquipe>(&mut pool)?
                .map_into())
        })
        .await
    }
}
