use std::error::Error;

use serde::{Deserialize, Serialize};

/// User objects allow you to manage a user and the user’s attributes such as a department, phone number, employee number, email, and username. The API allows you to create, delete, retrieve a user or a list of users, and update user information
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    /// The domo user id
    pub id: Option<u64>,

    /// User's full name
    pub name: Option<String>,

    /// User's primary email used in profile
    pub email: Option<String>,

    /// User's secondary email in profile
    pub alternate_email: Option<String>,

    /// Employee id within company
    pub employee_id: Option<String>,

    /// Employee number within company
    pub employee_number: Option<u64>,

    /// User's job title
    pub title: Option<String>,

    /// Primary phone number of user
    pub phone: Option<String>,

    /// Free text that can be used to define office location (e.g. City, State, Country)
    pub location: Option<String>,

    /// Free text that can be used to define department
    pub department: Option<String>,

    /// Time zone used to display to user the system times throughout Domo application
    pub timezone: Option<String>,

    /// Locale used to display to user the system settings throughout Domo application
    pub locale: Option<String>,

    /// The role of the user created (available roles are: 'Admin', 'Privileged', 'Participant')
    /// Deprecated in liu of custom roles and authorities
    pub role: Option<String>,

    /// The role id of the user created
    pub role_id: Option<u64>,

    /// If the user ID is related to a user that has been deleted, a subset of the user information will be returned, including a deleted property, which will be true.
    pub deleted: Option<bool>,
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            name: None,
            email: None,
            alternate_email: None,
            employee_number: None,
            employee_id: None,
            title: None,
            phone: None,
            location: None,
            department: None,
            timezone: None,
            locale: None,
            role: None,
            role_id: None,
            deleted: None,
        }
    }
    pub fn template() -> Self {
        User {
            id: Some(0),
            name: Some(String::from("First Last")),
            email: Some(String::from("First.Last@company.com")),
            alternate_email: Some(String::from("first.last@gmail.com")),
            employee_number: Some(0),
            employee_id: Some(String::from("employee id")),
            title: Some(String::from("Title")),
            phone: Some(String::from("+1 (800) 700-6000")),
            location: Some(String::from("CA")),
            department: Some(String::from("department")),
            timezone: Some(String::from("America/Los_Angeles")),
            locale: Some(String::from("en-US")),
            role: Some(String::from("Admin - Match roles defined in instance")),
            role_id: Some(0),
            deleted: Some(false),
        }
    }
}

/// User API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of users.
    pub async fn get_users(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<User>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        #[derive(Serialize)]
        struct QueryParams {
            pub limit: Option<u32>,
            pub offset: Option<u32>,
        }
        let q = QueryParams {
            limit,
            offset,
        };
        let mut response = surf::get(&format!("{}{}", self.host, "/v1/users"))
            .query(&q)?
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Fetch users by email in bulk
    pub async fn post_bulk_user_emails(
        &self,
        emails: &[String],
    ) -> Result<Vec<User>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        let mut response = surf::post(&format!("{}{}", self.host, "/v1/users/bulk/emails"))
            .header("Authorization", at)
            .body(surf::Body::from_json(&emails)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Creates a new user in your Domo instance.
    ///
    /// TODO param sendInvite=true
    pub async fn post_user(&self, user: User) -> Result<User, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        let mut response = surf::post(&format!("{}{}", self.host, "/v1/users"))
            .header("Authorization", at)
            .body(surf::Body::from_json(&user)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieves the details of an existing user.
    ///
    /// Returns a user object if valid user ID was provided. When requesting, if the user ID is related to a user that has been deleted, a subset of the user information will be returned, including a deleted property, which will be true.
    pub async fn get_user(&self, id: &str) -> Result<User, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        let mut response = surf::get(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Updates the specified user by providing values to parameters passed. Any parameter left out of the request will cause the specific user’s attribute to remain unchanged
    /// Currently all user fields are required
    pub async fn put_user(&self, id: &str, user: User) -> Result<User, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        let mut response = surf::put(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .body(surf::Body::from_json(&user)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Permanently deletes a user from your Domo instance
    /// This is destructive and cannot be reversed.
    pub async fn delete_user(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("user").await?;
        let mut response = surf::delete(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }
}
