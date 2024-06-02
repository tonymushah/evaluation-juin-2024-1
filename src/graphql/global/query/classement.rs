use async_graphql::{Context, Object};
use diesel::prelude::*;

use crate::{
    graphql::{objects::order::GraphQLOrdering, GetPoolConnection},
    models::{coureur_point::CoueurPoint, equipe_point::EquipePoint},
};

pub struct ClassementQueries;

#[Object]
impl ClassementQueries {
    pub async fn par_equipe(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
    ) -> crate::Result<Vec<EquipePoint>> {
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_equipe_point::dsl::*;
            Ok(match ordre {
                GraphQLOrdering::Ascending => v_equipe_point
                    .select(EquipePoint::as_select())
                    .order(points.asc())
                    .get_results(&mut pool)?,
                GraphQLOrdering::Descending => v_equipe_point
                    .select(EquipePoint::as_select())
                    .order(points.desc())
                    .get_results(&mut pool)?,
            })
        })
        .await
    }
    pub async fn par_coureur(
        &self,
        ctx: &Context<'_>,
        ordre: Option<GraphQLOrdering>,
    ) -> crate::Result<Vec<CoueurPoint>> {
        let ordre = ordre.unwrap_or(GraphQLOrdering::Descending);
        ctx.use_pool(move |mut pool| {
            use crate::view::v_coureur_point::dsl::*;
            Ok(match ordre {
                GraphQLOrdering::Ascending => v_coureur_point
                    .select(CoueurPoint::as_select())
                    .order(points.asc())
                    .get_results(&mut pool)?,
                GraphQLOrdering::Descending => v_coureur_point
                    .select(CoueurPoint::as_select())
                    .order(points.desc())
                    .get_results(&mut pool)?,
            })
        })
        .await
    }
}