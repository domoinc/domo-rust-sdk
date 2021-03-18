use domo::webhook::buzz::Message;
use domo::webhook::Client;

use serde_json::json;
use structopt::StructOpt;

use super::util;

/// Wraps domo webhook functionality
#[derive(StructOpt, Debug)]
pub enum WebhookCommand {
    #[structopt(name = "create-integration-message")]
    CreateIntegrationMessage {
        /// This is your webhook url.
        #[structopt(long = "url", env = "DOMO_INTEGRATION_WH_URL")]
        url: String,
        #[structopt(long = "token", env = "DOMO_INTEGRATION_WH_TOKEN")]
        token: String,
    },
    #[structopt(name = "create-buzz-message")]
    CreateBuzzMessage {
        /// This is your webhook url.
        #[structopt(long = "url", env = "DOMO_BUZZ_WH_URL")]
        url: String,
        title: Option<String>,
    },
    #[structopt(name = "create-dataset-json")]
    CreateDatasetJson {
        /// This is your webhook url.
        #[structopt(long = "url", env = "DOMO_DATASET_WH_URL")]
        url: String,
    },
}

pub async fn execute(editor: &str, command: WebhookCommand) {
    let c = Client::new();
    match command {
        WebhookCommand::CreateIntegrationMessage { url, token } => {
            let t = util::edit_md(editor, "Your message here").unwrap();
            c.post_integration_message(&url, &token, &t).await.unwrap();
        }
        WebhookCommand::CreateBuzzMessage { url, title } => {
            let t = util::edit_md(editor, "Your message here").unwrap();
            let m = Message { title, text: t };
            c.post_buzz_message(&url, m).await.unwrap();
        }
        WebhookCommand::CreateDatasetJson { url } => {
            let r = json!({
                "a": "Column A Value",
                "b": 43,
                "c": "Column C Value",
            });
            let r = util::edit_obj(editor, r, "").unwrap();
            c.post_dataset_json(&url, r).await.unwrap();
        }
    }
}
