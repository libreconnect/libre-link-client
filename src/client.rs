use crate::login::DataUser;
use crate::{
    connection::{ConnectionGraphResponse, RedirectResponse, ResponseConnections},
    glucose::{GlucoseHistoryRequest, LogBookRequest},
    login::try_get_access_data,
};
use reqwest::Client;
use sha2::{Digest, Sha256};

pub struct LibreLinkClient {
    client: Client,
    token: String,
    user_data: DataUser,
    base_url: Option<String>,
    region: Option<String>,
    credentials: Option<Credentials>,
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl LibreLinkClient {
    pub async fn new(
        credentials: Credentials,
        region: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let data = try_get_access_data(&credentials.username, &credentials.password).await;

        match data {
            Ok(data) => {
                let mut client = LibreLinkClient {
                    client: Client::new(),
                    user_data: data.user,
                    token: data.auth_ticket.token,
                    region,
                    base_url: None,
                    credentials: Some(credentials),
                };
                client.set_base_url();
                Ok(client)
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_token(token: String, user_id: String, region: Option<String>) -> Self {
        let mut client = LibreLinkClient {
            client: Client::new(),
            token,
            region,
            user_data: DataUser { id: user_id },
            base_url: None,
            credentials: None,
        };
        client.set_base_url();
        client
    }

    fn set_base_url(&mut self) {
        // if region is None then set https://api.libreview.io else set https://api-{region}.libreview.io
        if let Some(region) = &self.region {
            let url = format!("https://api-{}.libreview.io", region);
            self.base_url = Some(url);
        } else {
            let url = "https://api.libreview.io".to_string();
            self.base_url = Some(url);
        }
    }

    fn get_encoded_account_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.user_data.id.as_bytes());
        hex::encode(hasher.finalize())
    }

    async fn fetch_connections_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!("{}/{}", &base_url, "llu/connections");

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/17.4.1 Mobile/10A5355d Safari/8536.25")
            .header("version", "4.16.0")
            .header("product", "llu.ios")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("account-id", self.get_encoded_account_id())
            .bearer_auth(&self.token)
            .send()
            .await?;

        Ok(response.text().await?)
    }

    pub async fn get_connections(&mut self) -> Result<ResponseConnections, Box<dyn std::error::Error>> {
        let text = self.fetch_connections_raw().await?;
        // Check for redirect or error responses before parsing as connections
        if let Ok(redirect) = serde_json::from_str::<RedirectResponse>(&text) {
            if redirect.data.redirect {
                let creds = self.credentials.as_ref().ok_or_else(|| {
                    std::io::Error::other(format!(
                        "API requires regional redirect to '{}' but no credentials available to re-authenticate. \
                         Set LIBRELINK_REGION={} or use LibreLinkClient::new() instead of from_token().",
                        redirect.data.region, redirect.data.region
                    ))
                })?;

                // Re-authenticate against the correct regional endpoint
                let creds = creds.clone();
                let region = redirect.data.region.clone();
                let data = try_get_access_data(&creds.username, &creds.password).await?;
                self.token = data.auth_ticket.token;
                self.user_data = data.user;
                self.region = Some(region);
                self.set_base_url();

                let text = self.fetch_connections_raw().await?;
                let parsed: ResponseConnections = serde_json::from_str(&text)?;
                return Ok(parsed);
            }
        }

        let parsed: ResponseConnections = serde_json::from_str(&text)?;
        Ok(parsed)
    }

    pub async fn get_connection_graph(
        &self,
        connection_id: &str,
    ) -> Result<ConnectionGraphResponse, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}/{}/{}",
            &base_url, "llu/connections", connection_id, "graph"
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/17.4.1 Mobile/10A5355d Safari/8536.25")
            .header("version", "4.16.0")
            .header("product", "llu.ios")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("account-id", self.get_encoded_account_id())
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
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}?numPeriods={}&period={}",
            &base_url, "glucoseHistory", num_periods, period
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/17.4.1 Mobile/10A5355d Safari/8536.25")
            .header("version", "4.16.0")
            .header("product", "llu.ios")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("account-id", self.get_encoded_account_id())
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<GlucoseHistoryRequest, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_log_book(
        &self,
        connection_id: &str,
    ) -> Result<LogBookRequest, Box<dyn std::error::Error>> {
        let base_url = self.base_url.clone().unwrap();
        let url = format!(
            "{}/{}/{}/{}",
            &base_url, "llu/connections", connection_id, "logbook"
        );

        let response = self
            .client
            .get(url)
            .header("version", "4.16.0")
            .header("product", "llu.android")
            .header("User-Agent", "Apidog/1.0.0 (https://apidog.com)")
            .bearer_auth(&self.token)
            .send()
            .await?;

        let api_response: Result<LogBookRequest, reqwest::Error> = response.json().await;

        match api_response {
            Ok(response_data) => Ok(response_data),
            Err(e) => Err(Box::new(e)),
        }
    }
}
