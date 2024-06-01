use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::equipe_coureur;

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
