use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct County {
    id: u16,
    name: String,
    state: u8,
}
