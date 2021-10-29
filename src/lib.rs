pub struct Client {
    web_client: reqwest::Client,
}

impl Client {
    const API_URL: &'static str = "https://war-service-live.foxholeservices.com/api";

    pub fn new() -> Self {
        let web_client = reqwest::Client::new();

        Self { web_client }
    }

    pub async fn war_data(&self) -> String {
        const WAR_DATA: &'static str = "/worldconquest/war";

        let mut request_string = String::from(Self::API_URL);
        request_string.push_str(WAR_DATA);

        let response = self.web_client.get(request_string).send().await.unwrap();
        response.text().await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_war_data() {
        let client = Client::new();

        let response = client.war_data().await;
        println!("{}", response);
    }
}
