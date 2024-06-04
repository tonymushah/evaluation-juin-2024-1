use std::io::Read;

use csv::Reader;
use serde::Deserialize;

use crate::models::points::Points;

#[derive(Debug, Clone, Deserialize)]
pub struct PointsCSVData {
    pub classement: i32,
    pub points: i32,
}

impl From<PointsCSVData> for Points {
    fn from(value: PointsCSVData) -> Self {
        Self {
            rang: value.classement,
            valeur: value.points,
        }
    }
}

impl PointsCSVData {
    pub fn read<R: Read>(mut reader: Reader<R>) -> Vec<Self> {
        reader.deserialize::<PointsCSVData>().flatten().collect()
    }
}

#[cfg(test)]
mod points_csv_tests {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::PointsCSVData;
    #[test]
    fn read() -> anyhow::Result<()> {
        let buf_read = BufReader::new(File::open(
            "data/données importation juin 2024 - points.csv",
        )?);
        let reader = Reader::from_reader(buf_read);
        for entry in PointsCSVData::read(reader) {
            println!("{:#?}", entry);
        }
        Ok(())
    }
}
