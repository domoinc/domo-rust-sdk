use serde::{Deserialize, Serialize};

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
    pub async fn post_buzz_message(
        &self,
        url: &str,
        message: Message,
    ) -> Result<(), surf::Exception> {
        surf::post(&format!("{}", url)).body_json(&message)?.await?;
        Ok(())
    }
}
