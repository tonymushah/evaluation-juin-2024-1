use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::view::v_classement_categorie;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_classement_categorie)]
#[diesel(primary_key(coureur, categorie))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassementCategorie {
    pub coureur: i32,
    pub categorie: Uuid,
    pub points: BigDecimal,
    pub temps: BigDecimal,
}
