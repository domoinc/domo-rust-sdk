use serde::{Deserialize, Serialize};

impl super::Client {
    /// Allows a message to be posted back in response to an event
    /// Use fields off of the subscription to utilize this function
    pub async fn post_integration_message(
        &self,
        url: &str,
        token: &str,
        message: &str,
    ) -> Result<(), surf::Exception> {
        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default, rename_all = "camelCase")]
        struct C {
            text: String,
        }
        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default, rename_all = "camelCase")]
        struct M {
            content: C,
        }
        let m = M {
            content: C {
                text: format!("{}", message),
            },
        };
        surf::post(&format!("{}", url))
            .set_header("x-buzz-bot-token", token)
            .body_json(&m)?
            .await?;
        Ok(())
    }
}
