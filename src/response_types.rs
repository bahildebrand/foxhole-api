use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WarDataResponse {
    pub war_id: String,
    pub war_number: u32,
    pub winner: String,
    pub conquest_start_time: u64,
    pub conquest_end_time: Option<u64>,
    pub resistance_start_time: Option<u64>,
    pub required_victory_towns: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapDataResponse {
    pub maps: Vec<String>,
}
