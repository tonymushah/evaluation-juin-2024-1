use std::env;

use diesel::{dsl::sql, sql_types::Untyped, RunQueryDsl};

use crate::DbPool;

pub fn reset_db(con: &mut DbPool) -> crate::Result<()> {
    let user = env::var("DBUSER")?;
    sql::<Untyped>(format!("SELECT truncate_tables('{user}');").as_str())
        .execute(&mut con.get()?)?;
    Ok(())
}
