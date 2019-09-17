use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Message {
    /// An optional title to include with the message
    pub title: Option<String>,

    /// Markdown text
    pub text: String,
}

impl super::Client {
    /// A webhook can be created in the product.
    /// When created, a url can be obtained to post messages back into a channel
    pub fn post_buzz_message(&self, url: &str, message: Message) -> Result<(), Box<dyn Error>> {
        self.client
            .post(&format!("{}", url))
            .json(&message)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
