use std::io::Read;

use csv::Reader;
use diesel::{insert_into, prelude::*, PgConnection, QueryResult};
use serde::Deserialize;
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

use crate::models::{
    coureur::Coureur, equipe::Equipe, equipe_coureur::EquipeCoureur, temps_coureur::TempCoureur,
};

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Genre {
    #[serde(alias = "M")]
    Masculin,
    #[serde(alias = "F")]
    Feminin,
}

impl From<Genre> for i32 {
    fn from(value: Genre) -> Self {
        match value {
            Genre::Feminin => 2,
            Genre::Masculin => 1,
        }
    }
}

impl TryFrom<i32> for Genre {
    type Error = crate::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Masculin),
            2 => Ok(Self::Feminin),
            _ => Err(crate::Error::GenreParse),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultatCSV {
    pub etape_rang: i32,
    #[serde(alias = "numero dossard")]
    pub numero_dosard: i32,
    pub nom: String,
    pub genre: Genre,
    #[serde(alias = "date naissance", deserialize_with = "super::deserealize_date")]
    pub dtn: Date,
    pub equipe: String,
    #[serde(
        alias = "arrivée",
        deserialize_with = "super::deserealize_primitive_date_time"
    )]
    pub arrivee: PrimitiveDateTime,
}

impl ResultatCSV {
    pub fn read<R: Read>(mut reader: Reader<R>) -> Vec<Self> {
        reader.deserialize::<ResultatCSV>().flatten().collect()
    }
}

impl From<ResultatCSV> for Coureur {
    fn from(value: ResultatCSV) -> Self {
        Self {
            numero_dosard: value.numero_dosard,
            nom: value.nom,
            genre: value.genre.into(),
            dtn: value.dtn,
        }
    }
}

impl ResultatCSV {
    fn get_equipe_coeur_id(&self, pool: &mut PgConnection) -> QueryResult<Uuid> {
        use crate::schema::equipe_coureur::dsl::*;
        equipe_coureur
            .select(id_equipe_coureur)
            .filter(coureur.eq(self.numero_dosard))
            .get_result::<Uuid>(pool)
    }
    fn insert_equipe_coueur(&self, con: &mut PgConnection) -> QueryResult<Uuid> {
        let coureur_: Coureur = self.clone().into();
        {
            use crate::schema::coureur::dsl::*;
            insert_into(coureur)
                .values(&coureur_)
                .on_conflict(numero_dosard)
                .do_nothing()
                .execute(con)?;
        }
        let equipe_ = Equipe::get_by_name_or_insert(self.equipe.clone(), con)?;
        {
            use crate::schema::equipe_coureur::dsl::*;
            let ec = EquipeCoureur {
                id_equipe_coureur: Uuid::new_v4(),
                equipe: equipe_.id_equipe,
                coureur: coureur_.numero_dosard,
            };
            insert_into(equipe_coureur).values(&ec).execute(con)?;
            Ok(ec.id_equipe_coureur)
        }
    }
    fn get_equipe_coeur_id_or_insert(&self, con: &mut PgConnection) -> QueryResult<Uuid> {
        match self.get_equipe_coeur_id(con) {
            Ok(o) => Ok(o),
            Err(err) => {
                if let diesel::result::Error::NotFound = err {
                    self.insert_equipe_coueur(con)
                } else {
                    Err(err)
                }
            }
        }
    }
    fn get_temps(&self, con: &mut PgConnection) -> QueryResult<i32> {
        use crate::schema::etape::dsl::*;
        let depart_: PrimitiveDateTime = etape
            .select(depart)
            .filter(rang.eq(self.etape_rang))
            .get_result(con)?;
        let ar = self.arrivee - depart_;
        Ok(ar.as_seconds_f64().ceil() as i32)
    }
    pub fn insert(&self, con: &mut PgConnection) -> QueryResult<TempCoureur> {
        let input = TempCoureur {
            id_temps_coureur: Uuid::new_v4(),
            equipe_coureur: self.get_equipe_coeur_id_or_insert(con)?,
            etape: self.etape_rang,
            temps: Some(self.get_temps(con)?),
            points: None,
        };
        //println!("{:#?}", input);
        con.transaction(move |con| {
            use crate::schema::temps_coureur::dsl::*;
            insert_into(temps_coureur).values(&input).execute(con)?;
            Ok(input)
        })
    }
}

#[cfg(test)]
mod points_csv_tests {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::ResultatCSV;
    #[test]
    fn read() -> anyhow::Result<()> {
        let buf_read = BufReader::new(File::open(
            "data/données importation juin 2024 - resultat.csv",
        )?);
        let reader = Reader::from_reader(buf_read);
        for entry in ResultatCSV::read(reader) {
            println!("{:#?}", entry);
        }
        Ok(())
    }
}
