use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseLoginRequest {
    pub status: i32,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "authTicket")]
    pub auth_ticket: AuthTicket,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTicket {
    pub token: String,
    expires: u64,
    duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: i32,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub message: String,
}
