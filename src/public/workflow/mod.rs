use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

/// “Projects and Tasks” is a project management tool that helps you take real action with simple planning, assigning, and task-tracking features. You can create projects with various tasks and assignments. Those tasks exist within swim lanes or lists, and can be moved from list to list to show progress through a particular workflow. You can use default lists or create new custom lists. You can also add attachments to individual tasks to reference relevant materials and other artifacts.
/// Note: You will need to ensure that your client application has access to the Workflow scope in order to access the Projects and Tasks endpoints.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Project {
    /// The ID of the project
    pub id: Option<String>,

    /// The name of the project
    pub name: Option<String>,

    /// Description of the project
    pub description: Option<String>,

    /// The ID of the user who created the project
    pub created_by: Option<u64>,

    /// Date the project was created
    pub created_date: Option<DateTime<Utc>>,

    /// Due date of the project
    pub due_date: Option<DateTime<Utc>>,

    /// whether or not the project is a public project
    pub public: Option<bool>,

    /// The ID's of the members of the project
    pub members: Option<Vec<u64>>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            id: None,
            name: None,
            description: None,
            created_by: None,
            created_date: None,
            due_date: None,
            public: None,
            members: None,
        }
    }

    pub fn template() -> Self {
        Project {
            id: Some(String::from("0")),
            name: Some(String::from("Project Name")),
            description: Some(String::from("Project Description")),
            created_by: Some(12345),
            created_date: Some(Utc::now()),
            due_date: Some(Utc::now()),
            public: Some(true),
            members: Some(vec![0, 1, 2, 3]),
        }
    }
}

/// The list object
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct List {
    /// The ID of the List
    pub id: Option<u64>,

    /// The name of the List
    pub name: Option<String>,

    /// The type of List (i.e. TODO, WORKING_ON, COMPLETED)
    #[serde(rename = "type")]
    pub list_type: Option<String>,

    /// The ordered index of the list within the project.
    pub index: u32,
}

impl List {
    pub fn new() -> Self {
        List {
            id: None,
            name: None,
            list_type: None,
            index: 0,
        }
    }
    pub fn template() -> Self {
        List {
            id: Some(0),
            name: Some(String::from("List Name")),
            list_type: Some(String::from("")),
            index: 0,
        }
    }
}

/// The task object
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Task {
    /// The ID of the task
    pub id: Option<u64>,

    /// The ID of the project that the task belongs to
    pub project_id: Option<u64>,

    /// The ID of the list within a project that the task belongs to
    pub project_list_id: Option<u64>,

    /// The name of the task
    pub task_name: Option<String>,

    /// An optional description of the task
    pub description: Option<String>,

    /// The date the task was created
    pub created_date: Option<DateTime<Utc>>,

    /// The date the task is expected to be completed
    pub due_date: Option<DateTime<Utc>>,

    /// The prioritized order of the task in a list
    pub priority: Option<u32>,

    /// The ID of the Domo user that created the task
    pub created_by: Option<u64>,

    /// The ID of the Domo user that owns the task
    pub owned_by: Option<u64>,

    /// An array of user IDs that are assigned as contributors to the task
    pub contributors: Option<Vec<u64>>,

    /// The number of attachments that task has
    pub attachment_count: Option<u32>,

    /// An array of tags that have been assigned to the task
    pub tags: Option<Vec<String>>,

    /// whether or not the task has been archived
    pub archived: bool,
}

impl Task {
    pub fn new() -> Self {
        Task {
            id: None,
            project_id: None,
            project_list_id: None,
            task_name: None,
            description: None,
            created_date: None,
            due_date: None,
            priority: None,
            created_by: None,
            owned_by: None,
            contributors: None,
            attachment_count: None,
            tags: None,
            archived: false,
        }
    }
    pub fn template() -> Self {
        Task {
            id: Some(0),
            project_id: Some(0),
            project_list_id: Some(0),
            task_name: Some(String::from("Task Name")),
            description: Some(String::from("Task Description")),
            created_date: Some(Utc::now()),
            due_date: Some(Utc::now()),
            priority: Some(0),
            created_by: Some(27),
            owned_by: Some(27),
            contributors: Some(vec![0, 1, 2, 3]),
            attachment_count: Some(0),
            tags: Some(vec![
                String::from("A"),
                String::from("B"),
                String::from("C"),
            ]),
            archived: false,
        }
    }
}

/// The attachment object
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Attachment {
    /// The ID of the attachment
    pub id: Option<u32>,

    /// The ID of the task that the attachment belongs to
    pub task_id: Option<u32>,

    /// The date the attachment was created
    pub created_date: Option<DateTime<Utc>>,

    /// The filename of the attachment
    pub file_name: Option<String>,

    /// The mime type of the attachment
    pub mime_type: Option<String>,
}

/// Workflow API methods
/// Uses the form method_object
impl super::Client {
    /// Retrieves a list of all projects that the client scope has access to.
    pub fn get_projects(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/projects/"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Create a new project in your Domo instance
    ///
    /// Required attributes:
    /// * name
    /// * members
    /// * public
    pub fn post_project(&self, project: Project) -> Result<Project, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/projects"))
            .header("Authorization", at)
            .json(&project)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an individual existing project given a project id.
    /// Use the special project ID me to return your personal project.
    pub fn get_project(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!("{}{}{}", self.host, "/v1/projects/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Updates attributes of an existing project in your Domo instance. The following properties are read-only and cannot be updated with this request:
    /// * id
    /// * members
    /// * createdBy
    /// * createdDate
    ///
    /// Required attributes:
    /// * id
    pub fn put_project(&self, id: &str, project: Project) -> Result<Project, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .put(&format!("{}{}{}", self.host, "/v1/projects/", id))
            .header("Authorization", at)
            .json(&project)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes a project from your Domo instance.
    /// This is destructive and cannot be reversed.
    pub fn delete_project(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        self.client
            .delete(&format!("{}{}{}", self.host, "/v1/projects/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Retrieves a list of ids of the users that are members of the given project id.
    pub fn get_project_members(&self, id: &str) -> Result<Vec<u64>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}",
                self.host, "/v1/projects/", id, "/members"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Update the members of a given project id.
    pub fn put_project_members(&self, id: &str, members: Vec<u64>) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        self.client
            .put(&format!(
                "{}{}{}{}",
                self.host, "/v1/projects/", id, "/members"
            ))
            .header("Authorization", at)
            .json(&members)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Retrieves all lists available within a given project id.
    pub fn get_project_lists(&self, id: &str) -> Result<Vec<List>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}",
                self.host, "/v1/projects/", id, "/lists"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Creates a new list within the given project id.
    ///
    /// Required attributes:
    /// * project_id
    /// * name
    /// * type
    ///
    /// Index: Setting this property will re-order other lists in the project to maintain sequential order. Leaving this property blank will default the index to 1 and shift the index of all other lists.
    pub fn post_project_list(&self, project_id: &str, list: List) -> Result<List, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .post(&format!(
                "{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists"
            ))
            .header("Authorization", at)
            .json(&list)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an individual list given a project id and a list id.
    pub fn get_project_list(
        &self,
        project_id: &str,
        list_id: &str,
    ) -> Result<List, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Update the details of a list given an existing project id and list id.
    ///
    /// Required attributes:
    /// * project_id
    /// * list_id
    /// * name
    /// * type
    /// * index
    pub fn put_project_list(
        &self,
        project_id: &str,
        list_id: &str,
        list: List,
    ) -> Result<List, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .put(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id
            ))
            .header("Authorization", at)
            .json(&list)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes a list from your Domo instance.
    /// This is destructive and cannot be reversed.
    pub fn delete_project_list(
        &self,
        project_id: &str,
        list_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        self.client
            .delete(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Retrieves all tasks from a given project id.
    ///
    /// limit: The maximum amount of results to return (defaults to 10 with a maximum of 50)
    /// offset: The number of records to offset from the beginning of the result list (defaults to 0)
    pub fn get_project_tasks(
        &self,
        id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}",
                self.host, "/v1/projects/", id, "/tasks"
            ))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves all tasks from a given project id and list id
    ///
    /// limit: The maximum amount of results to return (defaults to 10 with a maximum of 50)
    /// offset: The number of records to offset from the beginning of the result list (defaults to 0)
    pub fn get_project_list_tasks(
        &self,
        project_id: &str,
        list_id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id, "/tasks"
            ))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Add a task to a project list.
    ///
    /// Required attributes:
    /// * projectId
    /// * projectListId
    /// * taskName
    /// * ownedBy
    /// * contributers
    pub fn post_project_list_task(
        &self,
        project_id: &str,
        list_id: &str,
        task: Task,
    ) -> Result<Task, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .post(&format!(
                "{}{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id, "/tasks"
            ))
            .header("Authorization", at)
            .json(&task)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves an individual task from a given project id and list id.
    pub fn get_project_list_task(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
    ) -> Result<Task, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id, "/tasks/", task_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Update the details of a task given an existing project id, list id, and task id.
    ///
    /// Required Attributes:
    /// * project_id
    /// * list_id
    /// * task_id
    /// * taskName
    /// * priority
    /// * ownedBy
    /// * contributors
    pub fn put_project_list_task(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
        task: Task,
    ) -> Result<Task, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .put(&format!(
                "{}{}{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id, "/tasks/", task_id
            ))
            .header("Authorization", at)
            .json(&task)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// TODO May just need to set the archived flag
    pub fn delete_project_list_task(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        self.client
            .delete(&format!(
                "{}{}{}{}{}{}{}",
                self.host, "/v1/projects/", project_id, "/lists/", list_id, "/tasks/", task_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Retrieve details about all of the attachments belonging to a particular task.
    pub fn get_project_list_task_attachments(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
    ) -> Result<Vec<Attachment>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}{}{}{}{}",
                self.host,
                "/v1/projects/",
                project_id,
                "/lists/",
                list_id,
                "/tasks/",
                task_id,
                "/attachments"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Downloads an individual attachment given an attachment id.
    pub fn get_project_list_task_attachment(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
        attachment_id: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        let mut buffer: Vec<u8> = Vec::new();
        self.client
            .get(&format!(
                "{}{}{}{}{}{}{}{}{}",
                self.host,
                "/v1/projects/",
                project_id,
                "/lists/",
                list_id,
                "/tasks/",
                task_id,
                "/attachments/",
                attachment_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .copy_to(&mut buffer)?;
        Ok(buffer)
    }

    /// Add a multipart form file to a task item as an attachment.
    pub fn post_project_list_task_attachment(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
        path: PathBuf,
    ) -> Result<Attachment, Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        let form = reqwest::multipart::Form::new().file("file", path).unwrap();
        Ok(self
            .client
            .post(&format!(
                "{}{}{}{}{}{}{}{}",
                self.host,
                "/v1/projects/",
                project_id,
                "/lists/",
                list_id,
                "/tasks/",
                task_id,
                "/attachments"
            ))
            .header("Authorization", at)
            .multipart(form)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes an attachment from your task.
    /// This is destructive and cannot be reversed.
    pub fn delete_project_list_task_attachment(
        &self,
        project_id: &str,
        list_id: &str,
        task_id: &str,
        attachment_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("workflow")?;
        self.client
            .delete(&format!(
                "{}{}{}{}{}{}{}{}{}",
                self.host,
                "/v1/projects/",
                project_id,
                "/lists/",
                list_id,
                "/tasks/",
                task_id,
                "/attachments/",
                attachment_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
