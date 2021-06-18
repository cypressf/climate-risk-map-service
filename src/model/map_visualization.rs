use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct MapVisualization {
    pub id: u64,
    pub dataset: u64,
    pub map_type: u8,
    pub legend_ticks: Option<u8>,
    pub should_normalize: bool,
    pub color_palette: u8,
    pub reverse_scale: bool,
    pub invert_normalized: bool,
    pub scale_type: u8,
    pub formatter_type: u8,
    pub decimals: u8,
    pub legend_formatter_type: Option<u8>,
    pub legend_decimals: Option<u8>,
}
