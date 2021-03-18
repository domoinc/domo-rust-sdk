pub mod account;
pub mod activity;
pub mod buzz;
pub mod dataset;
pub mod group;
pub mod page;
pub mod stream;
pub mod user;
pub mod workflow;

use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// All the calls to the public api return this error type on non-ok status codes (Also status codes are rarely consistent)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PubAPIError {
    pub status: u16,
    pub status_reason: Option<String>,
    pub message: String,
    pub path: Option<String>,
    pub toe: Option<String>,
}

impl std::error::Error for PubAPIError {}
impl std::fmt::Display for PubAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Domo Public API Error: {} {} {:?}",
            self.status,
            self.message,
            self.toe.as_ref()
        )
    }
}

/// The public API client.
/// All methods will be attached to this struct
pub struct Client {
    host: String,
    client_id: String,
    client_secret: String,
}

/// Client initialization and helper methods
impl Client {
    /// Create a new public api client
    pub fn new(host: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            host: String::from(host),
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        }
    }

    /// Trades the client_id and client_secret for an access token via the oauth2 token endpoint.
    async fn get_access_token(
        &self,
        scope: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let mut auth_basic_str = String::new();
        auth_basic_str.push_str(&self.client_id);
        auth_basic_str.push(':');
        auth_basic_str.push_str(&self.client_secret);
        let auth_basic = base64::encode(auth_basic_str);
        let mut response = surf::get(&format!("{}{}", self.host, "/oauth/token"))
            .query(&TokenQuery {
                grant_type: "client_credentials",
                scope,
            })?
            .header("Authorization", "Basic ".to_owned() + &auth_basic)
            .await?;
        if !response.status().is_success() {
            let e: Box<PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        let json: Value = response.body_json().await?;
        Ok(String::from("Bearer ") + json.get("access_token").unwrap().as_str().unwrap())
    }
}

#[derive(Serialize)]
struct TokenQuery<'a> {
    pub grant_type: &'a str,
    pub scope: &'a str,
}
