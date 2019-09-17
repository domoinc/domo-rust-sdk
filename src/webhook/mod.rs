pub mod buzz;
pub mod dataset;
pub mod integration;

use reqwest;

pub struct Client {
    client: reqwest::Client,
}

/// Client initialization and helper methods
impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
