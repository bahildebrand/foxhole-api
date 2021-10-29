pub mod response_types;

use response_types::WarDataResponse;

const WAR_DATA: &'static str = "/worldconquest/war";

pub struct Client {
    web_client: reqwest::Client,
}

impl Client {
    pub async fn war_data(&self) -> WarDataResponse {
        let request_string = build_request(WAR_DATA);

        let response = self.web_client.get(request_string).send().await.unwrap();
        response.json::<WarDataResponse>().await.unwrap()
    }
}

impl Default for Client {
    fn default() -> Self {
        let web_client = reqwest::Client::new();

        Self { web_client }
    }
}

fn build_request(endpoint: &'static str) -> String {
    #[cfg(not(test))]
    let mut request_string = "https://war-service-live.foxholeservices.com/api".to_string();

    #[cfg(test)]
    let mut request_string = mockito::server_url();

    request_string.push_str(endpoint);
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

        let response = client.war_data().await;
        assert_eq!(expected_response, response);
    }
}
