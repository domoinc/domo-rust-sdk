use domo::public::buzz::Integration;
use domo::public::buzz::Subscription;
use domo::public::Client;

use structopt::StructOpt;

use super::util;

/// Wraps the buzz api
#[derive(StructOpt, Debug)]
pub enum BuzzCommand {
    /// Retrieves a list of integrations that the client scope has access to.
    #[structopt(name = "list")]
    List {},

    /// Create a new integration
    #[structopt(name = "create")]
    CreateIntegration {},

    /// Retrieves an account by id
    #[structopt(name = "retrieve")]
    Retrieve { id: String },

    /// Permanently deletes a Buzz integration.
    #[structopt(name = "delete")]
    Delete { id: String },

    /// Retrieves an integration subscriptions
    #[structopt(name = "list-subscriptions")]
    ListSubscriptions { id: String },

    /// Create a new subscription
    #[structopt(name = "create-subscription")]
    CreateSubscription { id: String },

    /// Delete a subscription
    #[structopt(name = "delete-subscription")]
    DeleteSubscription { id: String, subscription_id: String },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: BuzzCommand) {
    match command {
        BuzzCommand::List {} => {
            let r = dc.get_integrations().await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        BuzzCommand::CreateIntegration {} => {
            let r = Integration::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_integration(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        BuzzCommand::Retrieve { id } => {
            let r = dc.get_integration(&id).await.unwrap();
            util::obj_template_output(r, template);
        }
        BuzzCommand::Delete { id } => {
            dc.delete_integration(&id).await.unwrap();
        }
        BuzzCommand::ListSubscriptions { id } => {
            let r = dc.get_integration_subscriptions(&id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        BuzzCommand::CreateSubscription { id } => {
            let r = Subscription::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_integration_subscription(&id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        BuzzCommand::DeleteSubscription {
            id,
            subscription_id,
        } => {
            dc.delete_integration_subscription(&id, &subscription_id)
                .await
                .unwrap();
        }
    }
}
