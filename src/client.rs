use reqwest::Client;

use crate::models::{
    connection::ResponseConnections,
    login::{ErrorResponse, ResponseLoginRequest},
};

pub struct LibreLinkClient {
    client: Client,
    base_url: String,
}

impl LibreLinkClient {
    pub fn new(base_url: &str) -> Self {
        LibreLinkClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn try_get_accesss_token(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, "llu/auth/login");

        let login_request = serde_json::json!({
            "email": username,
            "password": password,
        });

        let response = self
            .client
            .post(url)
            .header("version", "4.2.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .json(&login_request)
            .send()
            .await?;

        let text = response.text().await?;
        let api_response: Result<ResponseLoginRequest, serde_json::Error> =
            serde_json::from_str(&text);

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

    pub async fn get_connections(&self, token: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, "llu/connections");

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(token)
            .send()
            .await?;

        let api_response: Result<ResponseConnections, reqwest::Error> = response.json().await;

        println!("{:?}", api_response.unwrap());

        Ok(())
    }
}
