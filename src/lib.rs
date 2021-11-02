//! Foxhole War API.
//!
//! This crate is a wrapper around the [Foxhole War API](https://github.com/clapfoot/warapi). This
//! crate requires the use of async code, as well as the [Tokio](https://docs.rs/tokio) runtime.
//!
//! Usage example:
//!
//! ```
//! use foxhole_api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // The default shard is Live-1
//!     let client = Client::default();
//!
//!     let war_data = client.war_data().await.unwrap();
//!     let map_names = client.map_names().await.unwrap();
//!     let static_map_data = client.map_data_static("TheFingersHex".to_string()).await.unwrap();
//!     let dynamic_map_data = client.map_data_dynamic("TheFingersHex".to_string()).await.unwrap();
//! }
//! ```

pub mod response_types;

use std::error::Error;

use response_types::{MapDataResponse, MapNameResponse, WarDataResponse};
use serde::de::DeserializeOwned;

const MAP_NAME: &str = "/worldconquest/maps";
const WAR_DATA: &str = "/worldconquest/war";

/// Used to specify the shard of the api
pub enum Shard {
    Live1,
    Live2,
}

/// Client for fetching data from the war API.
///
/// This client contains an HTTP client, and only one instance should be needed per process.
pub struct Client {
    web_client: reqwest::Client,
    shard: Shard,
}

impl Client {
    pub fn new(shard: Shard) -> Self {
        let web_client = reqwest::Client::new();

        Self { web_client, shard }
    }

    /// Retrieves information about the current war.
    ///
    /// This endpoint retrieves information about the current war, and returns it deserialized as
    /// [`WarDataResponse`].
    pub async fn war_data(&self) -> Result<WarDataResponse, Box<dyn Error>> {
        let war_data: WarDataResponse = self.get_response(WAR_DATA.to_string()).await?;

        Ok(war_data)
    }

    /// Retrieves all map names.
    ///
    /// This endpoint retrieves all map names currently present, and returns them deserialized as
    /// [`MapNameResponse`].
    pub async fn map_names(&self) -> Result<MapNameResponse, Box<dyn Error>> {
        let maps: Vec<String> = self.get_response(MAP_NAME.to_string()).await?;
        let map_data = MapNameResponse { maps };

        Ok(map_data)
    }

    /// Retrieves all static map data.
    ///
    /// This endpoint retrieves all map data that will never change over the course of a war. This
    /// includes map text labels and resource node locations.
    pub async fn map_data_static(
        &self,
        map_name: String,
    ) -> Result<MapDataResponse, Box<dyn Error>> {
        // FIXME: Write a macro for this to avoid copy pasta
        let endpoint_string = format!("/worldconquest/maps/{}/static", map_name);
        let map_data: MapDataResponse = self.get_response(endpoint_string).await?;

        Ok(map_data)
    }

    /// Retrieves all dynamic map data.
    ///
    /// This endpoint retrieves all map daa that could change over the course of a war. This
    /// includes relic bases, and town halls that could change team ownership. Private data, such as
    /// player built fortifications, is not available.
    pub async fn map_data_dynamic(
        &self,
        map_name: String,
    ) -> Result<MapDataResponse, Box<dyn Error>> {
        // FIXME: Write a macro for this to avoid copy pasta
        let endpoint_string = format!("/worldconquest/maps/{}/dynamic/public", map_name);
        let map_data: MapDataResponse = self.get_response(endpoint_string).await?;

        Ok(map_data)
    }

    async fn get_response<T>(&self, endpoint: String) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let request_string = self.build_request(endpoint);
        let response = self.web_client.get(request_string.clone()).send().await?;
        let response = response.json::<T>().await?;

        Ok(response)
    }

    fn build_request(&self, endpoint: String) -> String {
        #[cfg(not(test))]
        let mut request_string = self.get_shard_url();

        #[cfg(test)]
        let mut request_string = mockito::server_url();

        request_string.push_str(endpoint.as_str());
        request_string
    }

    #[cfg(not(test))]
    fn get_shard_url(&self) -> String {
        let shard = match self.shard {
            Shard::Live1 => "live",
            Shard::Live2 => "live-2",
        };

        format!("https://war-service-{}.foxholeservices.com/api", shard)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new(Shard::Live1)
    }
}

#[cfg(test)]
mod test {
    use crate::response_types::{IconType, MapItem, MapTextItem, TeamId};

    use super::*;
    use mockito::{mock, Mock};

    fn build_mock(endpoint: &str, body: &'static str) -> Mock {
        mock("GET", endpoint)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_war_data() {
        let war_data_string = r#"{
            "warId" : "1e82269a-d82b-4350-b1b1-06a98c983503",
            "warNumber" : 83,
            "winner" : "NONE",
            "conquestStartTime" : 1632326703205,
            "conquestEndTime" : null,
            "resistanceStartTime" : null,
            "requiredVictoryTowns" : 32
        }"#;

        let expected_response = WarDataResponse {
            war_id: "1e82269a-d82b-4350-b1b1-06a98c983503".to_string(),
            war_number: 83,
            winner: TeamId::None,
            conquest_start_time: 1632326703205,
            conquest_end_time: None,
            resistance_start_time: None,
            required_victory_towns: 32,
        };

        let _m = build_mock(WAR_DATA, war_data_string);

        let client = Client::default();
        let response = client.war_data().await.unwrap();
        assert_eq!(expected_response, response);
    }

    #[tokio::test]
    async fn test_map_name() {
        let map_name_string = r#"[
            "TheFingersHex",
            "GreatMarchHex",
            "TempestIslandHex"
          ]"#;

        let _m = build_mock(MAP_NAME, map_name_string);

        let maps = vec![
            "TheFingersHex".to_string(),
            "GreatMarchHex".to_string(),
            "TempestIslandHex".to_string(),
        ];
        let expected_response = MapNameResponse { maps };

        let client = Client::default();
        let response = client.map_names().await.unwrap();
        assert_eq!(expected_response, response);
    }

    #[tokio::test]
    async fn test_map_data_static() {
        let map_data_string = r#"{
            "regionId": 38,
            "scorchedVictoryTowns": 0,
            "mapItems": [],
            "mapTextItems": [
              {
                "text": "Captain's Dread",
                "x": 0.8643478,
                "y": 0.4387644,
                "mapMarkerType": "Minor"
              },
              {
                "text": "Cavitatis",
                "x": 0.43523252,
                "y": 0.6119927,
                "mapMarkerType": "Minor"
              }
            ],
            "lastUpdated": 1635388391413,
            "version": 3
          }"#;

        let map_text_items = vec![
            MapTextItem {
                text: "Captain's Dread".to_string(),
                x: 0.8643478,
                y: 0.4387644,
                map_marker_type: "Minor".to_string(),
            },
            MapTextItem {
                text: "Cavitatis".to_string(),
                x: 0.43523252,
                y: 0.6119927,
                map_marker_type: "Minor".to_string(),
            },
        ];

        let expected_response = MapDataResponse {
            region_id: 38,
            scorched_victory_towns: 0,
            map_items: Vec::new(),
            map_text_items,
            last_updated: 1635388391413,
            version: 3,
        };

        // FIXME: Write a macro for this to avoid copy pasta
        let map_string = "TheFingersHex".to_string();
        let endpoint_string = format!("/worldconquest/maps/{}/static", map_string);
        let _m = build_mock(endpoint_string.as_str(), map_data_string);

        let client = Client::default();
        let response = client.map_data_static(map_string).await.unwrap();
        assert_eq!(expected_response, response);
    }

    #[tokio::test]
    async fn test_map_data_dynamic() {
        let map_data_string = r#"{
            "regionId" : 38,
            "scorchedVictoryTowns" : 0,
            "mapItems" : [ {
              "teamId" : "NONE",
              "iconType" : 20,
              "x" : 0.43503433,
              "y" : 0.83201146,
              "flags" : 0
            }, {
              "teamId" : "COLONIALS",
              "iconType" : 20,
              "x" : 0.83840775,
              "y" : 0.45411408,
              "flags" : 0
            }, {
              "teamId" : "WARDENS",
              "iconType" : 20,
              "x" : 0.83840775,
              "y" : 0.45411408,
                "flags" : 0
            } ],
            "mapTextItems" : [ ],
            "lastUpdated" : 1635534670643,
            "version" : 5
          }"#;

        let map_items = vec![
            MapItem {
                team_id: TeamId::None,
                icon_type: IconType::SalvageField,
                x: 0.43503433,
                y: 0.83201146,
                flags: 0,
            },
            MapItem {
                team_id: TeamId::Colonials,
                icon_type: IconType::SalvageField,
                x: 0.83840775,
                y: 0.45411408,
                flags: 0,
            },
            MapItem {
                team_id: TeamId::Wardens,
                icon_type: IconType::SalvageField,
                x: 0.83840775,
                y: 0.45411408,
                flags: 0,
            },
        ];

        let expected_response = MapDataResponse {
            region_id: 38,
            scorched_victory_towns: 0,
            map_items,
            map_text_items: Vec::new(),
            last_updated: 1635534670643,
            version: 5,
        };

        // FIXME: Write a macro for this to avoid copy pasta
        let map_string = "TheFingersHex".to_string();
        let endpoint_string = format!("/worldconquest/maps/{}/dynamic/public", map_string);
        let _m = build_mock(endpoint_string.as_str(), map_data_string);

        let client = Client::default();
        let response = client.map_data_dynamic(map_string).await.unwrap();
        assert_eq!(expected_response, response);
    }
}
