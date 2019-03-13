use serde::{Serialize, Deserialize};
use super::schema::palm_log;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name="palm_log"]
pub struct PalmLogEntry {
    pub log_time: String,
    pub moisture: i32
}
