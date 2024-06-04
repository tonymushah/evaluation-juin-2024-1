pub mod etape;
pub mod points;

use async_graphql::Enum;
use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};
use time::{format_description, Date, PrimitiveDateTime, Time};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum CSVDelimiters {
    Comma,
    Tabulation,
}

impl Default for CSVDelimiters {
    fn default() -> Self {
        Self::Comma
    }
}

pub trait DelimitCSVBuilder {
    fn delimit_csv_builder(&self, builder: &mut ReaderBuilder);
    fn new_builder(&self) -> ReaderBuilder {
        let mut builder = ReaderBuilder::default();
        self.delimit_csv_builder(&mut builder);
        builder
    }
}

impl DelimitCSVBuilder for CSVDelimiters {
    fn delimit_csv_builder(&self, builder: &mut ReaderBuilder) {
        match self {
            CSVDelimiters::Comma => builder.delimiter(b','),
            CSVDelimiters::Tabulation => builder.delimiter(b'\t'),
        };
    }
}

pub const IMPORT_DATE_TIME_FORMAT: &str = "[day]/[month]/[year] [hour]:[minute]:[second]";

pub const IMPORT_DATE_FORMAT: &str = "[day]/[month]/[year]";

pub const IMPORT_TIME_FORMAT: &str = "[hour]:[minute]:[second]";

fn deserealize_primitive_date_time<'de, D>(de: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    println!("{s}");
    let format =
        format_description::parse(IMPORT_DATE_TIME_FORMAT).map_err(serde::de::Error::custom)?;

    PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
}

fn deserealize_date<'de, D>(de: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    //println!("{s}");
    let format = format_description::parse(IMPORT_DATE_FORMAT).map_err(serde::de::Error::custom)?;
    Date::parse(&s, &format).map_err(serde::de::Error::custom)
}

fn deserealize_time<'de, D>(de: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    //println!("{s}");
    let format = format_description::parse(IMPORT_TIME_FORMAT).map_err(serde::de::Error::custom)?;
    Time::parse(&s, &format).map_err(serde::de::Error::custom)
}
