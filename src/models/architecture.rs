use crate::schema::*;
use crate::Connection;
use crate::DbId;
use anyhow::{Context, Result};
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "architecture"]
pub struct DbArchitecture {
    pub id: DbId,
    pub code: String,
}

impl DbArchitecture {
    pub fn get_architecute_id(conn: &Connection, arch: &str) -> Result<DbId> {
        let architecture_id = architecture::table
            .filter(architecture::code.eq(arch))
            .select(architecture::id)
            .first::<DbId>(conn)
            .context("Error loading architecture from DB")?; // todo return 404
        Ok(architecture_id)
    }
}
