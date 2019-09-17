pub mod account;
pub mod activity;
pub mod buzz;
pub mod dataset;
pub mod group;
pub mod page;
pub mod stream;
pub mod user;
pub mod workflow;

use reqwest;
use serde_json::Value;
use std::error::Error;

pub struct Client {
    host: String,
    client_id: String,
    client_secret: String,
    client: reqwest::Client,
}

/// Client initialization and helper methods
impl Client {
    pub fn new(host: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            host: String::from(host),
            client: reqwest::Client::new(),
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        }
    }

    fn get_access_token(&self, scope: &str) -> Result<String, Box<dyn Error>> {
        let json: Value = self
            .client
            .get(&format!("{}{}", self.host, "/oauth/token"))
            .query(&[("grant_type", "client_credentials"), ("scope", scope)])
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()?
            .error_for_status()?
            .json()?;
        Ok(String::from("Bearer ") + &json.get("access_token").unwrap().as_str().unwrap())
    }
}
