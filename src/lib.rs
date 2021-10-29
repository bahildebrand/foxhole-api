pub mod response_types;

use std::error::Error;

use response_types::{MapNameResponse, WarDataResponse};
use serde::de::DeserializeOwned;

use crate::response_types::MapDataResponse;

const MAP_NAME: &str = "/worldconquest/maps";
const WAR_DATA: &str = "/worldconquest/war";

pub struct Client {
    web_client: reqwest::Client,
}

impl Client {
    pub async fn war_data(&self) -> Result<WarDataResponse, Box<dyn Error>> {
        let war_data: WarDataResponse = self.get_response(WAR_DATA.to_string()).await?;

        Ok(war_data)
    }

    pub async fn map_names(&self) -> Result<MapNameResponse, Box<dyn Error>> {
        let maps: Vec<String> = self.get_response(MAP_NAME.to_string()).await?;
        let map_data = MapNameResponse { maps };

        Ok(map_data)
    }

    pub async fn map_data_static(
        &self,
        map_name: String,
    ) -> Result<MapDataResponse, Box<dyn Error>> {
        // FIXME: Write a macro for this to avoid copy past
        let endpoint_string = format!("/worldconquest/maps/{}/static", map_name);
        let map_data: MapDataResponse = self.get_response(endpoint_string).await?;

        Ok(map_data)
    }

    pub async fn map_data_dynamic(
        &self,
        map_name: String,
    ) -> Result<MapDataResponse, Box<dyn Error>> {
        // FIXME: Write a macro for this to avoid copy past
        let endpoint_string = format!("/worldconquest/maps/{}/dynamic/public", map_name);
        let map_data: MapDataResponse = self.get_response(endpoint_string).await?;

        Ok(map_data)
    }

    async fn get_response<T>(&self, endpoint: String) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let request_string = build_request(endpoint);
        let response = self.web_client.get(request_string.clone()).send().await?;
        let response = response.json::<T>().await?;

        Ok(response)
    }
}

impl Default for Client {
    fn default() -> Self {
        let web_client = reqwest::Client::new();

        Self { web_client }
    }
}

fn build_request(endpoint: String) -> String {
    #[cfg(not(test))]
    let mut request_string = "https://war-service-live.foxholeservices.com/api".to_string();

    #[cfg(test)]
    let mut request_string = mockito::server_url();

    request_string.push_str(endpoint.as_str());
    request_string
}

#[cfg(test)]
mod test {
    use crate::response_types::{MapItem, MapTextItem};

    use super::*;
    use mockito::mock;

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
            winner: "NONE".to_string(),
            conquest_start_time: 1632326703205,
            conquest_end_time: None,
            resistance_start_time: None,
            required_victory_towns: 32,
        };

        let _m = mock("GET", WAR_DATA)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(war_data_string)
            .create();

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

        let _m = mock("GET", MAP_NAME)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(map_name_string)
            .create();

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
        let _m = mock("GET", endpoint_string.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(map_data_string)
            .create();

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
              "teamId" : "NONE",
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
                team_id: "NONE".to_string(),
                icon_type: 20,
                x: 0.43503433,
                y: 0.83201146,
                flags: 0,
            },
            MapItem {
                team_id: "NONE".to_string(),
                icon_type: 20,
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

        let _m = mock("GET", endpoint_string.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(map_data_string)
            .create();

        let client = Client::default();
        let response = client.map_data_dynamic(map_string).await.unwrap();
        assert_eq!(expected_response, response);
    }
}
