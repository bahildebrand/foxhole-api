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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_war_data_deserialize() {
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

        let response: WarDataResponse = serde_json::from_str(&war_data_string).unwrap();
        assert_eq!(expected_response, response);
    }
}
