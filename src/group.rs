use domo::public::group::Group;
use domo::public::Client;

use structopt::StructOpt;

use super::util;

/// Wraps the group api
#[derive(StructOpt, Debug)]
pub enum GroupCommand {
    /// Get a list of all groups in your Domo instance.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },
    /// Creates a new group in your Domo instance.
    #[structopt(name = "create")]
    CreateGroup {},
    /// Retrieves the details of an existing group.
    #[structopt(name = "retrieve")]
    Retrieve { id: String },
    /// Updates the specified group by providing values to parameters passed.
    /// Any parameter left out of the request will cause the specific group’s attribute to remain unchanged.
    #[structopt(name = "update")]
    UpdateGroup { id: String },
    /// Permanently deletes a group from your Domo instance.
    #[structopt(name = "delete")]
    DeleteGroup { id: String },
    /// List the users in a group in your Domo instance.
    #[structopt(name = "list-users")]
    ListUsers { id: String },
    /// Add user to a group in your Domo instance.
    #[structopt(name = "add-user")]
    AddUser { group_id: String, user_id: String },
    /// Remove a user from a group in your Domo instance.
    #[structopt(name = "remove-user")]
    RemoveUser { group_id: String, user_id: String },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: GroupCommand) {
    match command {
        GroupCommand::List { limit, offset } => {
            let r = dc.get_groups(limit, offset).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        GroupCommand::CreateGroup {} => {
            let r = Group::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_group(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        GroupCommand::Retrieve { id } => {
            let r = dc.get_group(&id).await.unwrap();
            util::obj_template_output(r, template);
        }
        GroupCommand::UpdateGroup { id } => {
            let r = dc.get_group(&id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.put_group(&id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        GroupCommand::DeleteGroup { id } => {
            dc.delete_group(&id).await.unwrap();
        }
        GroupCommand::ListUsers { id } => {
            let r = dc.get_group_users(&id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        GroupCommand::AddUser { group_id, user_id } => {
            dc.put_group_user(&group_id, &user_id).await.unwrap();
        }
        GroupCommand::RemoveUser { group_id, user_id } => {
            dc.delete_group_user(&group_id, &user_id).await.unwrap();
        }
    }
}
