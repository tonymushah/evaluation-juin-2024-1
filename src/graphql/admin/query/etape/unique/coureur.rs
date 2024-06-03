use async_graphql::{ComplexObject, Context, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    graphql::GetPoolConnection,
    models::{coureur::Coureur, equipe::Equipe},
    view::v_temps_coureur_etape_equipe_coureur,
};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[graphql(complex)]
#[diesel(table_name = v_temps_coureur_etape_equipe_coureur)]
#[diesel(primary_key(equipe_coureur))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EtapeCoureur {
    #[graphql(skip)]
    pub coureur: i32,
    pub temps: Option<i32>,
    pub equipe_coureur: Uuid,
    pub points: Option<i32>,
    #[graphql(skip)]
    pub equipe: Uuid,
}

#[ComplexObject]
impl EtapeCoureur {
    pub async fn coureur(&self, ctx: &Context<'_>) -> crate::Result<Coureur> {
        let id = self.coureur;
        ctx.use_pool(move |mut pool| {
            use crate::schema::coureur::dsl::*;
            Ok(coureur
                .select(Coureur::as_select())
                .filter(numero_dosard.eq(id))
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn equipe(&self, ctx: &Context<'_>) -> crate::Result<Equipe> {
        let id = self.equipe;
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(Equipe::as_select())
                .filter(id_equipe.eq(id))
                .get_result(&mut pool)?)
        })
        .await
    }
}
