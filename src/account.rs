use domo::public::account::Account;
use domo::public::Client;

use std::collections::HashMap;

use structopt::StructOpt;

use super::util;

/// Wraps the account api
#[derive(StructOpt, Debug)]
pub enum AccountCommand {
    /// Get a list of all Accounts for which the user has permissions.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },
    /// When creating an Account, you must specify the Account Type properties.
    /// The Account Type properties are different, depending on the type of Account you are trying to create.
    /// To retrieve which Account Type properties to specify, use the GET /v1/accounts/account-types/{ACCOUNT_TYPE_ID} endpoint.
    #[structopt(name = "create")]
    Create {
        /// The account type to use when creating this account
        account_type: String,
    },
    /// Retrieves an account by id
    #[structopt(name = "retrieve")]
    Retrieve { id: String },
    /// Updates the specified Account’s metadata as well as the Account’s Type properties.
    #[structopt(name = "update")]
    Update { id: String },
    /// Deletes an Account from your Domo instance.
    #[structopt(name = "delete")]
    Delete { id: String },
    /// Share an Account with a User.
    #[structopt(name = "share")]
    Share { account_id: String, user_id: u64 },
    /// Get a list of all Account Types for which the user has permissions.
    #[structopt(name = "list-types")]
    ListTypes {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },
    /// Retrieves an account type by id
    #[structopt(name = "retrieve-type")]
    RetrieveType { id: String },
}

pub fn execute(dc: Client, e: &str, t: Option<String>, command: AccountCommand) {
    match command {
        AccountCommand::List { limit, offset } => {
            let r = dc.get_accounts(limit, offset).unwrap();
            util::vec_obj_template_output(r, t);
        }
        AccountCommand::Create { account_type } => {
            //Go get the account type and then populate the template accordingly
            let mut r = Account::template();
            let mut at = dc.get_account_type(&account_type).unwrap();
            //Pre-pop the property fields from the default template
            if let Some(ref hm) = at.templates {
                if hm.contains_key("default") {
                    let mut properties: HashMap<String, String> = HashMap::new();
                    for p in hm.get("default").unwrap().properties.as_ref().unwrap() {
                        properties.insert(
                            String::from(p.name.as_ref().unwrap()),
                            format!("TODO: {}", p.prompt.as_ref().unwrap()),
                        );
                    }
                    at.properties = Some(properties);
                }
            }
            r.account_type = Some(at);
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_account(r).unwrap();
            util::obj_template_output(r, t);
        }
        AccountCommand::Retrieve { id } => {
            let r = dc.get_account(&id).unwrap();
            util::obj_template_output(r, t);
        }
        AccountCommand::Update { id } => {
            let r = dc.get_account(&id).unwrap();
            let r = util::edit_obj(e, r, "").unwrap();
            dc.patch_account(&id, r).unwrap();
        }
        AccountCommand::Delete { id } => {
            dc.delete_account(&id).unwrap();
        }
        AccountCommand::Share {
            account_id,
            user_id,
        } => {
            dc.post_account_share(&account_id, user_id).unwrap();
        }
        AccountCommand::ListTypes { limit, offset } => {
            let r = dc.get_account_types(limit, offset).unwrap();
            util::vec_obj_template_output(r, t);
        }
        AccountCommand::RetrieveType { id } => {
            let r = dc.get_account_type(&id).unwrap();
            util::obj_template_output(r, t);
        }
    }
}
