use std::{
    fs::File,
    io::{BufWriter, Write},
};

use backend::graphql::admin::AdminSchema as Schema;
use evaluation_juin_2024_1 as backend;

fn main() {
    let schema = Schema::default();
    let mut file = BufWriter::new(File::create("app/schemas/admin.graphqls").unwrap());
    file.write_all(schema.sdl().as_bytes()).unwrap();
    file.flush().unwrap();
}
