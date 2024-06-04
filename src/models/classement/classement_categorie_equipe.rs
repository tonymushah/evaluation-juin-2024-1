use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{models::coureur_point::CoueurPoint, view::v_classement_categories_equipe};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_classement_categories_equipe)]
#[diesel(primary_key(coureur, categorie, equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassementCategorieEquipe {
    pub coureur: i32,
    pub categorie: Uuid,
    pub equipe: Uuid,
    pub points: BigDecimal,
    pub temps: BigDecimal,
}

impl From<ClassementCategorieEquipe> for CoueurPoint {
    fn from(value: ClassementCategorieEquipe) -> Self {
        Self {
            coureur: value.coureur,
            points: value.points,
            temps: value.temps,
        }
    }
}
