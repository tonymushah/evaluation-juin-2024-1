use async_graphql::{Context, Object};
use diesel::{expression::expression_types::NotSelectable, pg::Pg, prelude::*};
use uuid::Uuid;

use crate::{
    graphql::{objects::order::GraphQLOrdering, GetPoolConnection, OffsetLimit, ResultsData},
    models::{
        classement::classement_categorie::ClassementCategorie, coureur_point::CoueurPoint,
        equipe_point::EquipePoint, Paginate,
    },
};

pub struct ClassementQueries;

#[Object]
impl ClassementQueries {
    pub async fn par_equipe(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
        #[graphql(default)] pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<EquipePoint>> {
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_equipe_point::dsl::*;
            Ok(match ordre {
                GraphQLOrdering::Ascending => v_equipe_point
                    .select(EquipePoint::as_select())
                    .order(points.asc())
                    .paginate_with_param(pagination)
                    .to_results_data(&mut pool)?,
                GraphQLOrdering::Descending => v_equipe_point
                    .select(EquipePoint::as_select())
                    .order(points.desc())
                    .paginate_with_param(pagination)
                    .to_results_data(&mut pool)?,
            })
        })
        .await
    }
    pub async fn par_coureur(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
        #[graphql(default)] pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<CoueurPoint>> {
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_coureur_point::dsl::*;
            Ok(match ordre {
                GraphQLOrdering::Ascending => v_coureur_point
                    .select(CoueurPoint::as_select())
                    .order(points.asc())
                    .paginate_with_param(pagination)
                    .to_results_data(&mut pool)?,
                GraphQLOrdering::Descending => v_coureur_point
                    .select(CoueurPoint::as_select())
                    .order(points.desc())
                    .paginate_with_param(pagination)
                    .to_results_data(&mut pool)?,
            })
        })
        .await
    }
    pub async fn par_categorie(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
        #[graphql(default)] pagination: OffsetLimit,
        id: Uuid,
    ) -> crate::Result<ResultsData<CoueurPoint>> {
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_classement_categorie::dsl::*;
            let orde: Box<
                dyn BoxableExpression<v_classement_categorie, Pg, SqlType = NotSelectable>,
            > = match ordre {
                GraphQLOrdering::Ascending => Box::new(points.asc()),
                GraphQLOrdering::Descending => Box::new(points.desc()),
            };
            Ok(v_classement_categorie
                .order(orde)
                .filter(categorie.eq(id))
                .select(ClassementCategorie::as_select())
                .paginate_with_param(pagination)
                .to_results_data::<ClassementCategorie>(&mut pool)?
                .map_into())
        })
        .await
    }
}
