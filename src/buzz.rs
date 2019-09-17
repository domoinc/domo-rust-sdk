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

pub fn execute(dc: Client, e: &str, t: Option<String>, command: BuzzCommand) {
    match command {
        BuzzCommand::List {} => {
            let r = dc.get_integrations().unwrap();
            util::vec_obj_template_output(r, t);
        }
        BuzzCommand::CreateIntegration {} => {
            let r = Integration::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_integration(r).unwrap();
            util::obj_template_output(r, t);
        }
        BuzzCommand::Retrieve { id } => {
            let r = dc.get_integration(&id).unwrap();
            util::obj_template_output(r, t);
        }
        BuzzCommand::Delete { id } => {
            dc.delete_integration(&id).unwrap();
        }
        BuzzCommand::ListSubscriptions { id } => {
            let r = dc.get_integration_subscriptions(&id).unwrap();
            util::vec_obj_template_output(r, t);
        }
        BuzzCommand::CreateSubscription { id } => {
            let r = Subscription::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_integration_subscription(&id, r).unwrap();
            util::obj_template_output(r, t);
        }
        BuzzCommand::DeleteSubscription {
            id,
            subscription_id,
        } => {
            dc.delete_integration_subscription(&id, &subscription_id)
                .unwrap();
        }
    }
}
