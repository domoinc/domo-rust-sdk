use serde::{Deserialize, Serialize};
use std::error::Error;

/// Group objects allow you to manage a group and users associated to a group.
/// Groups allow you to set access rights, send Buzz messages, or share content that stays consistent even when the group members may change.
/// The API allows you to create, delete, retrieve a user or a list of users, and update user information.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Group {
    /// The ID of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,

    /// The name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Whether the group is the default group
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,

    /// The active status of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// The ID of the user who created the group
    /// TODO This should be a u64 reference for consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    creator_id: Option<String>,

    /// The member count of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    member_count: Option<u32>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            id: None,
            name: None,
            default: None,
            active: None,
            creator_id: None,
            member_count: None,
        }
    }

    pub fn template() -> Self {
        Group {
            id: Some(0),
            name: Some(String::from("Group Name")),
            default: Some(false),
            active: Some(true),
            creator_id: Some(String::from("0")),
            member_count: Some(0),
        }
    }
}

/// Group API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of all groups in your Domo instance.
    /// Returns all group objects that meet argument criteria from original request.
    pub fn get_groups(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Group>, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/groups"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Creates a new group in your Domo instance.
    /// Returns a group object when successful.
    /// The returned group will have user attributes based on the information that was provided when group was created.
    pub fn post_group(&self, group: Group) -> Result<Group, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/groups"))
            .header("Authorization", at)
            .json(&group)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an existing group.
    /// Returns a group object if valid group ID was provided.
    /// When requesting, if the group ID is related to a customer that has been deleted, a subset of the group's information will be returned, including a deleted property, which will be true.
    pub fn get_group(&self, id: &str) -> Result<Group, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .get(&format!("{}{}{}", self.host, "/v1/groups/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Updates the specified group by providing values to parameters passed.
    /// Any parameter left out of the request will cause the specific group’s attribute to remain unchanged.
    /// Returns the parameter of success or error based on the group ID being valid.
    pub fn put_group(&self, id: &str, group: Group) -> Result<Group, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .put(&format!("{}{}{}", self.host, "/v1/groups/", id))
            .header("Authorization", at)
            .json(&group)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes a group from your Domo instance.
    /// This is destructive and cannot be reversed.
    pub fn delete_group(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        self.client
            .delete(&format!("{}{}{}", self.host, "/v1/groups/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// List the users in a group in your Domo instance.
    /// Returns IDs of users that are a part of the requested group.
    pub fn get_group_users(&self, id: &str) -> Result<Vec<u64>, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .get(&format!("{}{}{}{}", self.host, "/v1/groups/", id, "/users"))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Add user to a group in your Domo instance.
    pub fn put_group_user(&self, group_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        self.client
            .put(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/groups/", group_id, "/users/", user_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Remove a user from a group in your Domo instance.
    pub fn delete_group_user(&self, group_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        self.client
            .delete(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/groups/", group_id, "/users/", user_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
