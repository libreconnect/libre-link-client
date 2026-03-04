use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseLoginRequest {
    pub status: i32,
    pub data: LoginData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    #[serde(rename = "authTicket")]
    pub auth_ticket: AuthTicket,
    pub user: DataUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataUser {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTicket {
    pub token: String,
    pub expires: u64,
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: i32,
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub message: String,
}

pub async fn try_get_access_data(
    username: &str,
    password: &str,
) -> Result<LoginData, Box<dyn std::error::Error>> {
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

    if text.is_empty() || text.eq_ignore_ascii_case("{}") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No response from Libre Link",
        )))
    }
    let api_response: Result<ResponseLoginRequest, serde_json::Error> = serde_json::from_str(&text);

    match api_response {
        Ok(response_data) => Ok(response_data.data),
        Err(_) => {
            let error_response: ErrorResponse = serde_json::from_str(&text).unwrap();

            let message: String = error_response
                .error
                .as_ref()
                .map(|e| e.message.clone())
                .unwrap_or_else(|| "Unknown error".to_string());

            if error_response.status == 4 {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Privacy policy acceptance required, first login using the Libre Linkup app",
                )))
            } else if error_response.status == 2 {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    message,
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
