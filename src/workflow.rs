use domo::public::workflow::{List, Project, Task};
use domo::public::Client;

use std::io;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use super::util;

/// Wraps the workflow api
#[derive(StructOpt, Debug)]
pub enum WorkflowCommand {
    /// Retrieves a list of projects that the client scope has access to.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },

    /// Create a new project
    #[structopt(name = "create")]
    Create {},

    /// Retrieves the details of an individual existing project given a project id.
    #[structopt(name = "retrieve")]
    Retrieve { project_id: String },

    /// Update a project
    #[structopt(name = "update")]
    Update { project_id: String },

    /// Permanently deletes a project from your Domo instance.
    #[structopt(name = "delete")]
    Delete { project_id: String },

    /// Retrieves all tasks from a given project id.
    #[structopt(name = "list-tasks")]
    ListTasks {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
        project_id: String,
    },

    /// Retrieves a list of ids of the users that are members of the given project id.
    #[structopt(name = "list-members")]
    ListMembers { project_id: String },

    /// Retrieves all lists available within a given project id.
    #[structopt(name = "list-lists")]
    ListLists { project_id: String },

    /// Create a new list
    #[structopt(name = "create-list")]
    CreateList { project_id: String },

    /// Retrieves the details of an individual list given a project id and a list id.
    #[structopt(name = "retrieve-list")]
    RetrieveList { project_id: String, list_id: String },

    /// Update a list
    #[structopt(name = "update-list")]
    UpdateList { project_id: String, list_id: String },

    /// Permanently deletes a list from your Domo instance.
    #[structopt(name = "delete-list")]
    DeleteList { project_id: String, list_id: String },

    /// Retrieves all tasks from a given project id and list id
    #[structopt(name = "list-tasks")]
    ListListTasks {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
        project_id: String,
        list_id: String,
    },

    /// Create a new task
    #[structopt(name = "create-task")]
    CreateListTask { project_id: String, list_id: String },

    /// Retrieves an individual task from a given project id and list id.
    #[structopt(name = "retrieve-task")]
    RetrieveListTask {
        project_id: String,
        list_id: String,
        task_id: String,
    },

    /// Update a list
    #[structopt(name = "update-task")]
    UpdateListTask {
        project_id: String,
        list_id: String,
        task_id: String,
    },

    /// Delete a task from a list in a project
    #[structopt(name = "delete-task")]
    DeleteListTask {
        project_id: String,
        list_id: String,
        task_id: String,
    },

    /// Retrieve details about all of the attachments belonging to a particular task.
    #[structopt(name = "list-attachments")]
    ListListTaskAttachments {
        project_id: String,
        list_id: String,
        task_id: String,
    },

    /// Downloads an individual attachment given an attachment id.
    #[structopt(name = "download-attachment")]
    DownloadListTaskAttachment {
        project_id: String,
        list_id: String,
        task_id: String,
        attachment_id: String,
    },

    /// Add a multipart form file to a task item as an attachment.
    #[structopt(name = "upload-attachment")]
    UploadListTaskAttachment {
        project_id: String,
        list_id: String,
        task_id: String,
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },

    /// Permanently deletes an attachment from your task.
    #[structopt(name = "delete-attachment")]
    DeleteListTaskAttachment {
        project_id: String,
        list_id: String,
        task_id: String,
        attachment_id: String,
    },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: WorkflowCommand) {
    match command {
        WorkflowCommand::List { limit, offset } => {
            let r = dc.get_projects(limit, offset).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::Create {} => {
            let r = Project::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_project(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::Retrieve { project_id } => {
            let r = dc.get_project(&project_id).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::Update { project_id } => {
            let r = dc.get_project(&project_id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.put_project(&project_id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::Delete { project_id } => {
            dc.delete_project(&project_id).await.unwrap();
        }
        WorkflowCommand::ListMembers { project_id } => {
            let r = dc.get_project_members(&project_id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::ListLists { project_id } => {
            let r = dc.get_project_lists(&project_id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::CreateList { project_id } => {
            let r = List::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_project_list(&project_id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::RetrieveList {
            project_id,
            list_id,
        } => {
            let r = dc.get_project_list(&project_id, &list_id).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::UpdateList {
            project_id,
            list_id,
        } => {
            let r = dc.get_project_list(&project_id, &list_id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.put_project_list(&project_id, &list_id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::DeleteList {
            project_id,
            list_id,
        } => {
            dc.delete_project_list(&project_id, &list_id).await.unwrap();
        }
        WorkflowCommand::ListTasks {
            project_id,
            limit,
            offset,
        } => {
            let r = dc
                .get_project_tasks(&project_id, limit, offset)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::ListListTasks {
            project_id,
            list_id,
            limit,
            offset,
        } => {
            let r = dc
                .get_project_list_tasks(&project_id, &list_id, limit, offset)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::CreateListTask {
            project_id,
            list_id,
        } => {
            let r = Task::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc
                .post_project_list_task(&project_id, &list_id, r)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::RetrieveListTask {
            project_id,
            list_id,
            task_id,
        } => {
            let r = dc
                .get_project_list_task(&project_id, &list_id, &task_id)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::UpdateListTask {
            project_id,
            list_id,
            task_id,
        } => {
            let r = dc
                .get_project_list_task(&project_id, &list_id, &task_id)
                .await
                .unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc
                .put_project_list_task(&&project_id, &list_id, &task_id, r)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::DeleteListTask {
            project_id,
            list_id,
            task_id,
        } => {
            dc.delete_project_list_task(&project_id, &list_id, &task_id)
                .await
                .unwrap();
        }
        WorkflowCommand::ListListTaskAttachments {
            project_id,
            list_id,
            task_id,
        } => {
            let r = dc
                .get_project_list_task_attachments(&project_id, &list_id, &task_id)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
        WorkflowCommand::DownloadListTaskAttachment {
            project_id,
            list_id,
            task_id,
            attachment_id,
        } => {
            let r = dc
                .get_project_list_task_attachment(&project_id, &list_id, &task_id, &attachment_id)
                .await
                .unwrap();
            io::stdout().write_all(&r).unwrap();
        }
        WorkflowCommand::UploadListTaskAttachment {
            project_id,
            list_id,
            task_id,
            file,
        } => {
            let r = dc
                .post_project_list_task_attachment(&project_id, &list_id, &task_id, file)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        WorkflowCommand::DeleteListTaskAttachment {
            project_id,
            list_id,
            task_id,
            attachment_id,
        } => {
            dc.delete_project_list_task_attachment(&project_id, &list_id, &task_id, &attachment_id)
                .await
                .unwrap();
        }
    }
}
