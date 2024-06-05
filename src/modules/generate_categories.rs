use std::{error::Error, fmt::Display};

use diesel::{dsl::insert_into, prelude::*, QueryResult};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::{categorie::Categorie, coureur::Coureur, coureur_categorie::CoureurCategorie};

use super::imports::resultat::Genre;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemCategories {
    Homme,
    Femme,
    Senior,
}

impl From<SystemCategories> for Uuid {
    fn from(value: SystemCategories) -> Self {
        match value {
            SystemCategories::Homme => {
                Uuid::parse_str("a947640f-6ce6-4401-934c-091f6a8ec54c").unwrap()
            }
            SystemCategories::Femme => {
                Uuid::parse_str("ae57ea02-d1c6-42e6-9a12-35944bcfdbef").unwrap()
            }
            SystemCategories::Senior => {
                Uuid::parse_str("343579ff-80ce-45d6-b135-01aa580faf78").unwrap()
            }
        }
    }
}

impl From<SystemCategories> for Categorie {
    fn from(value: SystemCategories) -> Self {
        Self {
            id_categorie: value.into(),
            designation: value.title(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SystemCategoriesParseError;

impl Display for SystemCategoriesParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cannot parse the given type to the prebuilt categories")
    }
}

impl Error for SystemCategoriesParseError {}

impl TryFrom<Uuid> for SystemCategories {
    type Error = SystemCategoriesParseError;
    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        if value == Self::Femme.into() {
            Ok(Self::Femme)
        } else if value == Self::Homme.into() {
            Ok(Self::Homme)
        } else if value == Self::Senior.into() {
            Ok(Self::Senior)
        } else {
            Err(SystemCategoriesParseError)
        }
    }
}

impl SystemCategories {
    pub fn title(&self) -> String {
        match self {
            SystemCategories::Homme => String::from("Homme"),
            SystemCategories::Femme => String::from("Femme"),
            SystemCategories::Senior => String::from("Senior"),
        }
    }
    pub fn validate(&self, coureur: &Coureur) -> bool {
        let now = OffsetDateTime::now_utc().year();
        match self {
            SystemCategories::Homme => coureur.genre == Into::<i32>::into(Genre::Masculin),
            SystemCategories::Femme => coureur.genre == Into::<i32>::into(Genre::Feminin),
            SystemCategories::Senior => now - coureur.dtn.year() >= 18,
        }
    }
    pub fn all() -> Vec<Self> {
        vec![Self::Homme, Self::Femme, Self::Senior]
    }
    pub fn seed(con: &mut PgConnection) -> QueryResult<Vec<Categorie>> {
        use crate::schema::categorie::dsl::*;
        let all: Vec<Categorie> = Self::all().into_iter().map(|c| c.into()).collect();
        insert_into(categorie)
            .values(&all)
            .on_conflict(id_categorie)
            .do_nothing()
            .execute(con)?;
        Ok(all)
    }
    pub fn attribute_coureur(
        &self,
        coursier: &Coureur,
        con: &mut PgConnection,
    ) -> Option<CoureurCategorie> {
        use crate::schema::coureur_categorie::dsl::*;
        if self.validate(coursier) {
            insert_into(coureur_categorie)
                .values(&CoureurCategorie {
                    id_coureur_categorie: Uuid::new_v4(),
                    categorie: (*self).into(),
                    coureur: coursier.numero_dosard,
                })
                .returning(CoureurCategorie::as_returning())
                .get_result(con)
                .ok()
        } else {
            None
        }
    }
    pub fn attributes_coureur(coursier: &Coureur, con: &mut PgConnection) -> Vec<CoureurCategorie> {
        let mut cc = Vec::<CoureurCategorie>::new();
        for sys in Self::all() {
            if let Some(cc_) = sys.attribute_coureur(coursier, con) {
                cc.push(cc_);
            }
        }
        cc
    }
    pub fn attributes_coureurs<'a>(
        coursies: impl Iterator<Item = &'a Coureur>,
        con: &mut PgConnection,
    ) -> Vec<CoureurCategorie> {
        let mut cc = Vec::<CoureurCategorie>::new();
        for coursier in coursies {
            cc.append(&mut Self::attributes_coureur(coursier, con));
        }
        cc
    }
    pub fn generate_categories(con: &mut PgConnection) -> QueryResult<Vec<CoureurCategorie>> {
        use crate::schema::coureur::dsl::*;
        let coureurs = coureur.select(Coureur::as_select()).get_results(con)?;
        Ok(Self::attributes_coureurs(coureurs.iter(), con))
    }
}
