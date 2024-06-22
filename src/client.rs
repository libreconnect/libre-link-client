use reqwest::Client;

use crate::{
    connection::{ConnectionGraphResponse, ResponseConnections},
    glucose::GlucoseHistoryRequest,
    login::try_get_access_token,
};

pub struct LibreLinkClient {
    client: Client,
    token: String,
    base_url: String,
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl LibreLinkClient {
    pub async fn new(credentials: Credentials) -> Result<Self, Box<dyn std::error::Error>> {
        let token = try_get_access_token(&credentials.username, &credentials.password).await;

        match token {
            Ok(token) => Ok(LibreLinkClient {
                client: Client::new(),
                token: token,
                base_url: "https://api.libreview.io".to_string(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn from_token(token: String) -> Self {
        LibreLinkClient {
            client: Client::new(),
            token: token,
            base_url: "https://api.libreview.io".to_string(),
        }
    }

    pub async fn get_connections(&self) -> Result<ResponseConnections, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", &self.base_url, "llu/connections");

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<ResponseConnections, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_connection_graph(
        &self,
        connection_id: &str,
    ) -> Result<ConnectionGraphResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}/{}/{}",
            &self.base_url, "llu/connections", connection_id, "graph"
        );

        let response = self
            .client
            .get(url)
            .header("version", "4.7.1")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<ConnectionGraphResponse, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_glucose_history(
        &self,
        num_periods: i32,
        period: i32,
    ) -> Result<GlucoseHistoryRequest, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}?numPeriods={}&period={}",
            &self.base_url, "glucoseHistory", num_periods, period
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<GlucoseHistoryRequest, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }
}
