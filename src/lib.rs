pub mod response_types;

use std::error::Error;

use response_types::{MapDataResponse, WarDataResponse};
use serde::de::DeserializeOwned;

const MAP_DATA: &str = "/worldconquest/maps";
const WAR_DATA: &str = "/worldconquest/war";

pub struct Client {
    web_client: reqwest::Client,
}

impl Client {
    pub async fn war_data(&self) -> Result<WarDataResponse, Box<dyn Error>> {
        let war_data: WarDataResponse = self.get_response(WAR_DATA.to_string()).await?;

        Ok(war_data)
    }

    pub async fn map_data(&self) -> Result<MapDataResponse, Box<dyn Error>> {
        let maps: Vec<String> = self.get_response(MAP_DATA.to_string()).await?;
        let map_data = MapDataResponse { maps };

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
    async fn test_map_data() {
        let map_data_string = r#"[
            "TheFingersHex",
            "GreatMarchHex",
            "TempestIslandHex"
          ]"#;

        let _m = mock("GET", MAP_DATA)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(map_data_string)
            .create();

        let maps = vec![
            "TheFingersHex".to_string(),
            "GreatMarchHex".to_string(),
            "TempestIslandHex".to_string(),
        ];
        let expected_response = MapDataResponse { maps };

        let client = Client::default();
        let response = client.map_data().await.unwrap();
        assert_eq!(expected_response, response);
    }
}
