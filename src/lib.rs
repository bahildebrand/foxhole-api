pub mod response_types;

use response_types::WarDataResponse;

pub struct Client {
    web_client: reqwest::Client,
}

impl Client {
    pub async fn war_data(&self) -> WarDataResponse {
        const WAR_DATA: &str = "/worldconquest/war";

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
    const API_URL: &str = "https://war-service-live.foxholeservices.com/api";

    let mut request_string = String::from(API_URL);
    request_string.push_str(endpoint);

    request_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_war_data() {
        let client = Client::default();

        let response = client.war_data().await;
        println!("{:?}", response);
    }
}
