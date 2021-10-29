use serde;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

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
    pub icon_type: IconType,
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

#[derive(Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u16)]
pub enum IconType {
    StaticBase = 5,
    StaticBase2 = 6,
    StaticBase3 = 7,
    ForwardBase1 = 8,
    ForwardBase2 = 9,
    ForwardBase3 = 10,
    Hospital = 11,
    VehicleFactory = 12,
    Armory = 13,
    SupplyStation = 14,
    Workshop = 15,
    ManufacturingPlant = 16,
    Refinery = 17,
    Shipyard = 18,
    TechCenter = 19,
    SalvageField = 20,
    ComponentField = 21,
    FuelField = 22,
    SulfurField = 23,
    WorldMapTent = 24,
    TravelTent = 25,
    TrainingArea = 26,
    SpecialBase = 27,
    ObservationTower = 28,
    Fort = 29,
    TroopShip = 30,
    SulfurMine = 32,
    StorageFacility = 33,
    Factory = 34,
    GarrisonStation = 35,
    AmmoFactory = 36,
    RocketSite = 37,
    SalvageMine = 38,
    ConstructionYard = 39,
    ComponentMine = 40,
    OilWell = 41,
    RelicBase1 = 45,
    RelicBase2 = 46,
    RelicBase3 = 47,
    MassProductionFactory = 51,
    Seaport = 52,
    CoastalGun = 53,
    SoulFactory = 54,
    TownBase1 = 56,
    TownBase2 = 57,
    TownBase3 = 58,
}
