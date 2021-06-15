use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Data {
    pub county_id: u16,
    pub state_id: u8,
    pub value: f32,
}
