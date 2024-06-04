pub mod points;

use async_graphql::Enum;
use csv::ReaderBuilder;

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
