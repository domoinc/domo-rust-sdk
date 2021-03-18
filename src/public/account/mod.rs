use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

/// The Account API allows you to create, update, validate and share accounts in Domo.
/// If you would like to manage a large number of accounts at scale from agencies or other 3rd party vendors that you currently manage individually through the Data Center in Domo, the Accounts API makes that possible.
/// Note – The Accounts API will only return information for accounts you own or for accounts that have been shared with you in Domo.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Account {
    /// The ID of the Account
    pub id: Option<String>,

    /// The name of the Account
    pub name: Option<String>,

    /// Indicates if the Account needs to be re-authorized or not.
    pub valid: Option<bool>,

    /// The type of the Account
    #[serde(rename = "type")]
    pub account_type: Option<AccountType>,
}

impl Account {
    pub fn new() -> Self {
        Account {
            id: None,
            name: None,
            valid: None,
            account_type: None,
        }
    }
    pub fn template() -> Self {
        Account {
            id: Some(String::from("0")),
            name: Some(String::from("Account Name")),
            valid: Some(true),
            account_type: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountType {
    /// The ID of the Account Type
    pub id: Option<String>,

    /// The name of the Account Type
    pub name: Option<String>,

    /// The properties of the Account
    pub properties: Option<HashMap<String, String>>,

    /// The Template that defines the properties required to create an Account of this type
    #[serde(rename = "_templates")]
    pub templates: Option<HashMap<String, AccountTemplate>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountTemplate {
    /// The name of the Account Type
    pub name: Option<String>,

    /// The title of the Template
    pub title: Option<String>,

    /// The content-type header when sending the request to create an Account
    pub content_type: Option<String>,

    /// The HTTP method used when creating the Account
    pub method: Option<String>,

    /// The properties available to be sent when creating an Account
    pub properties: Option<Vec<Property>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Property {
    /// The property name
    pub name: Option<String>,

    /// The prompt used when filling in this property
    pub prompt: Option<String>,

    /// The regex that defines a valid value for this property
    pub regex: Option<String>,

    /// Whether this property is required or not
    pub required: Option<bool>,
}

#[derive(Serialize)]
struct ListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Account API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of all Accounts for which the user has permissions.
    /// Returns all Accounts objects that meet argument criteria from original request.
    pub async fn get_accounts(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Account>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let q = ListParams { limit, offset };
        let mut response = surf::get(&format!("{}{}", self.host, "/v1/accounts"))
            .query(&q)?
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Create an Account
    /// When creating an Account, you must specify the Account Type properties.
    /// The Account Type properties are different, depending on the type of Account you are trying to create.
    /// To retrieve which Account Type properties to specify, use the GET /v1/accounts/account-types/{ACCOUNT_TYPE_ID} endpoint.
    /// Returns an Account object when successful.
    /// The returned object will not contain any properties within the Account Type object.
    pub async fn post_account(
        &self,
        account: Account,
    ) -> Result<Account, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let mut response = surf::post(&format!("{}{}", self.host, "/v1/accounts"))
            .header("Authorization", at)
            .body(surf::Body::from_json(&account)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieves the details of an existing account.
    /// Returns an Account object if a valid Account ID was provided.
    /// When requesting, the Account Type object will not contain any properties.
    pub async fn get_account(
        &self,
        id: &str,
    ) -> Result<Account, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let mut response = surf::get(&format!("{}{}{}", self.host, "/v1/accounts/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Updates the specified Account’s metadata as well as the Account’s Type properties.
    /// Returns the updated Account.
    ///
    /// TODO: Should probably return the updated object for consistency
    pub async fn patch_account(
        &self,
        id: &str,
        account: Account,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let mut response = surf::patch(&format!("{}{}{}", self.host, "/v1/accounts/", id))
            .header("Authorization", at)
            .body(surf::Body::from_json(&account)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Deletes an Account from your Domo instance.
    pub async fn delete_account(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let mut response = surf::delete(&format!("{}{}{}", self.host, "/v1/accounts/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Share an Account with a User.
    pub async fn post_account_share(
        &self,
        account_id: &str,
        user_id: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        // The User to share the Account with.
        // Only the User's id attribute is required.
        // See the Users API for more information.
        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default, rename_all = "camelCase")]
        struct User {
            id: u64,
        }
        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default)]
        struct Share {
            user: User,
        }
        let obj: Share = Share {
            user: User { id: user_id },
        };
        let mut response = surf::post(&format!(
            "{}{}{}{}",
            self.host, "/v1/accounts/", account_id, "/shares"
        ))
        .header("Authorization", at)
        .body(surf::Body::from_json(&obj)?)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Get a list of all Account Types for which the user has permissions.
    /// Returns all Account Type objects that meet argument criteria from original request.
    pub async fn get_account_types(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<AccountType>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let q = ListParams { limit, offset };
        let mut response = surf::get(&format!("{}{}", self.host, "/v1/account-types"))
            .query(&q)?
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieve the details of an account type.
    /// This includes information on the properties required to create an Account of this type.
    /// Returns an Account Type object if valid Account Type ID was provided.
    pub async fn get_account_type(
        &self,
        id: &str,
    ) -> Result<AccountType, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("account").await?;
        let mut response = surf::get(&format!("{}{}{}", self.host, "/v1/account-types/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }
}
