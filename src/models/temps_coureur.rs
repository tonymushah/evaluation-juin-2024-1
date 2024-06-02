use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::temps_coureur;

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
#[graphql(input_name = "TempCoureurInput")]
#[diesel(table_name = temps_coureur)]
#[diesel(primary_key(id_temps_coureur))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TempCoureur {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_temps_coureur: Uuid,
    pub temps: Option<i32>,
    pub points: Option<i32>,
    pub equipe_coureur: Uuid,
    pub etape: i32,
}
