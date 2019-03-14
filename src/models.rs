use serde::{Serialize, Deserialize};
use super::schema::*;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name="palm_log"]
pub struct PalmLogEntry {
    pub log_time: String,
    pub moisture: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub date: String
}
