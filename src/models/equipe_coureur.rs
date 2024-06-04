use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use diesel::prelude::*;
use time::Date;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, schema::equipe_coureur, view::v_equipe_coureur};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Identifiable,
    Selectable,
    Insertable,
    Queryable,
    AsChangeset,
    SimpleObject,
    InputObject,
)]
#[graphql(input_name = "EquipeCoureurInput")]
#[diesel(table_name = equipe_coureur)]
#[diesel(primary_key(id_equipe_coureur))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipeCoureur {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_equipe_coureur: Uuid,
    pub coureur: i32,
    pub equipe: Uuid,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[graphql(complex)]
#[diesel(table_name = v_equipe_coureur)]
#[diesel(primary_key(coureur, equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VEquipeCoureur {
    pub coureur: i32,
    pub equipe: Uuid,
    pub nom_coureur: String,
    pub genre: i32,
    pub dtn: Date,
    #[graphql(skip)]
    pub pseudo_equipe: String,
    pub nom_equipe: String,
    pub id_equipe_coureur: Uuid,
}

impl VEquipeCoureur {
    pub fn by_coureur(dosard: i32, con: &mut PgConnection) -> QueryResult<Self> {
        use crate::view::v_equipe_coureur::dsl::*;
        v_equipe_coureur
            .select(Self::as_select())
            .filter(coureur.eq(dosard))
            .get_result(con)
    }
}

#[ComplexObject]
impl VEquipeCoureur {
    pub async fn points(&self, ctx: &Context<'_>) -> crate::Result<Option<i64>> {
        let dosard = self.coureur;
        ctx.use_pool(move |mut pool| {
            use crate::view::v_coureur_point::dsl::*;
            Ok(v_coureur_point
                .select(points)
                .filter(coureur.eq(dosard))
                .get_result::<i64>(&mut pool)
                .ok())
        })
        .await
    }
}
