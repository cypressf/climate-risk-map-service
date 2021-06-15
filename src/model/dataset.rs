use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Dataset {
    pub id: u64,
    pub short_name: String,
    pub name: String,
    pub description: String,
    pub units: String,
}
