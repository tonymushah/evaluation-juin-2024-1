use async_graphql::{InputObject, SimpleObject};
use convert_case::{Case, Casing};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::equipe;

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
#[graphql(input_name = "EquipeInput")]
#[diesel(table_name = equipe)]
#[diesel(primary_key(id_equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Equipe {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_equipe: Uuid,
    pub pseudo: String,
    #[graphql(skip)]
    pub mot_passe: String,
    pub nom: String,
}

impl Equipe {
    pub fn get_by_name(_nom: String, con: &mut PgConnection) -> QueryResult<Self> {
        use crate::schema::equipe::dsl::*;
        equipe
            .filter(nom.eq(_nom))
            .select(Self::as_select())
            .get_result(con)
    }
    fn insert_by_name(_nom: String, con: &mut PgConnection) -> QueryResult<Self> {
        use crate::schema::equipe::dsl::*;
        let insert = Self {
            id_equipe: Uuid::new_v4(),
            pseudo: _nom.to_case(Case::Snake),
            mot_passe: _nom.to_case(Case::Camel),
            nom: _nom,
        };
        diesel::insert_into(equipe).values(&insert).execute(con)?;
        Ok(insert)
    }
    pub fn get_by_name_or_insert(nom: String, con: &mut PgConnection) -> QueryResult<Self> {
        match Self::get_by_name(nom.clone(), con) {
            Ok(o) => Ok(o),
            Err(err) => {
                if let diesel::result::Error::NotFound = err {
                    Self::insert_by_name(nom, con)
                } else {
                    Err(err)
                }
            }
        }
    }
}
