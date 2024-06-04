use std::io::Read;

use bigdecimal::{BigDecimal, FromPrimitive};
use csv::Reader;
use serde::{Deserialize, Deserializer};
use time::{Date, PrimitiveDateTime, Time};

use crate::models::etape::Etape;

#[derive(Debug, Clone, Deserialize)]
pub struct EtapeCSVDATA {
    pub etape: String,
    #[serde(deserialize_with = "deserealize_decimale")]
    pub longueur: BigDecimal,
    #[serde(alias = "nb coureur")]
    pub coureurs: i32,
    pub rang: i32,
    #[serde(alias = "date départ", deserialize_with = "super::deserealize_date")]
    pub depart_date: Date,
    #[serde(alias = "heure départ")]
    #[serde(deserialize_with = "super::deserealize_time")]
    pub depart_heure: Time,
}

fn deserealize_decimale<'de, D>(de: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(de)?;
    s = s.replace(',', ".");
    //println!("{s}");
    BigDecimal::from_f64(s.parse::<f64>().map_err(serde::de::Error::custom)?)
        .ok_or(serde::de::Error::custom("cannot convert f64 to BigDecimal"))
}

impl From<EtapeCSVDATA> for Etape {
    fn from(value: EtapeCSVDATA) -> Self {
        Self {
            rang: value.rang,
            longueur: value.longueur,
            nom: value.etape,
            nb_coureur_par_equipe: value.coureurs,
            depart: PrimitiveDateTime::new(value.depart_date, value.depart_heure),
            finished: None,
        }
    }
}

impl EtapeCSVDATA {
    pub fn read<R: Read>(mut reader: Reader<R>) -> Vec<Self> {
        reader.deserialize::<EtapeCSVDATA>().flatten().collect()
    }
}

#[cfg(test)]
mod points_csv_tests {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::EtapeCSVDATA;
    #[test]
    fn read() -> anyhow::Result<()> {
        let buf_read = BufReader::new(File::open(
            "data/données importation juin 2024 - etape.csv",
        )?);
        let reader = Reader::from_reader(buf_read);
        for entry in EtapeCSVDATA::read(reader) {
            println!("{:#?}", entry);
        }
        Ok(())
    }
}
