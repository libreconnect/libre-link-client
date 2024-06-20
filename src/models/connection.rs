use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    id: String,
    #[serde(rename = "patientId")]
    pub patient_id: String,
    country: String,
    status: i32,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "targetLow")]
    target_low: i32,
    #[serde(rename = "targetHigh")]
    target_high: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseConnections {
    pub status: i32,
    pub data: Vec<Connection>,
}
