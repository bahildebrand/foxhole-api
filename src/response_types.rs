//! Response types for all war api endpoints.
//!
//! For more detailed information on the contents of each response see the
//! [Foxhole War API](https://github.com/clapfoot/warapi) definition.

use serde;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// Response for the `worldconquest/war` endpoint.
///
/// This response contains information about the status of the war for a given shard.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WarDataResponse {
    pub war_id: String,
    pub war_number: u32,
    pub winner: TeamId,
    pub conquest_start_time: u64,
    pub conquest_end_time: Option<u64>,
    pub resistance_start_time: Option<u64>,
    pub required_victory_towns: u8,
}

/// Response for the `worldconquest/maps` endpoint.
///
/// Contains a list of all maps present on the shard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapNameResponse {
    pub maps: Vec<String>,
}

/// Response for the /worldconquest/maps/{map}/dynamic|static endpoints.
///
/// Contains information about a requested map hex. This includes static objects, as well as objects
/// that can change dynamically. This does not include player built fortifications.
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

/// Contains information about a single item present on a map hex.
///
/// Includes the owner of the item, its position, as well as what type of item it is.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapItem {
    pub team_id: TeamId,
    pub icon_type: IconType,
    pub x: f32,
    pub y: f32,
    pub flags: u16,
}

/// Contains information about a map label.
///
/// Includes the text, position, and type of a label.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapTextItem {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub map_marker_type: MapMarkerType,
}

/// The type of icon for a map item.
///
/// This includes a mapping of the icon type to the underlying number used by the API.
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
    StormCannon = 59,
    IntelCenter = 60,
}

/// Team id for a map item.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamId {
    None,
    Wardens,
    Colonials,
}

/// Indicates whether or not a map marker is major or minor.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum MapMarkerType {
    Major,
    Minor,
}
