use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct State {
    pub id: u8,
    pub name: String,
}
