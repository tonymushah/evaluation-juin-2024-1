use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use time::Date;
use uuid::Uuid;

use crate::{schema::equipe_coureur, view::v_equipe_coureur};

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
#[diesel(table_name = v_equipe_coureur)]
#[diesel(primary_key(coureur, equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VEquipeCoureur {
    pub coureur: i32,
    pub equipe: Uuid,
    pub nom_coureur: String,
    pub genre: i32,
    pub dtn: Date,
    pub pseudo_equipe: String,
    pub nom_equipe: String,
}
