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

pub async fn try_get_access_token(
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.libreview.io/llu/auth/login";

    let login_request = serde_json::json!({
        "email": username,
        "password": password,
    });

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("version", "4.2.1")
        .header("product", "llu.android")
        .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
        .json(&login_request)
        .send()
        .await?;

    let text = response.text().await?;
    let api_response: Result<ResponseLoginRequest, serde_json::Error> = serde_json::from_str(&text);

    match api_response {
        Ok(response_data) => Ok(response_data.data.auth_ticket.token),
        Err(_) => {
            let error_response: ErrorResponse = serde_json::from_str(&text).unwrap();
            if error_response.status == 2 {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    error_response.error.message,
                )))
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unknown error",
                )))
            }
        }
    }
}
