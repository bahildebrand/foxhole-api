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
pub struct MapNameResponse {
    pub maps: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapDataResponse {
    pub region_id: u16,
    pub scorched_victory_towns: u16,
    pub map_items: Vec<MapItem>,
    pub map_text_items: Vec<MapTextItem>,
    pub last_updated: u64,
    pub version: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapItem {
    pub team_id: String,
    pub icon_type: u16,
    pub x: f32,
    pub y: f32,
    pub flags: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapTextItem {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub map_marker_type: String,
}
