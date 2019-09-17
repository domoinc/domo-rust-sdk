use serde::{Deserialize, Serialize};
use std::error::Error;

impl super::Client {
    /// Allows a message to be posted back in response to an event
    /// Use fields off of the subscription to utilize this function
    pub fn post_integration_message(
        &self,
        url: &str,
        token: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error>> {
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
        self.client
            .post(&format!("{}", url))
            .header("x-buzz-bot-token", token)
            .json(&m)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
