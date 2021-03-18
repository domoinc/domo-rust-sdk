use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

/// A Buzz integration is a service hosted outside of Domo’s infrastructure that can receive events from Buzz, and can post messages to Buzz. To use this feature, invoke this API to register an integration, then create one or more event subscriptions for the integration. When a corresponding event occur, Buzz will POST an HTTP request using the configured URL and headers.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Integration {
    /// The integration id
    pub id: Option<String>,

    /// The name of the integration, shown whenever it posts a message to Buzz
    pub name: Option<String>,

    /// Description of the integration
    pub description: Option<String>,

    /// The scope of the integration. Values can be:
    /// * PUBLIC_CHANNELS All public channels
    /// * OWNER_ACCESS All channels accessible by the user that creates the integration
    /// * CHANNEL_LIST All channels specified in the channelIds list
    pub scope: Option<String>,

    /// Must be provided if and only if scope is CHANNEL_LIST
    pub channel_ids: Option<Vec<String>>,

    /// Every event that Buzz sends to the integration will be sent as an HTTP POST with these headers. Headers may be used for authentication or other purposes.
    pub headers: Option<Vec<Header>>,
}

impl Integration {
    pub fn new() -> Self {
        Integration {
            id: None,
            name: None,
            description: None,
            scope: None,
            channel_ids: None,
            headers: None,
        }
    }

    pub fn template() -> Self {
        Integration {
            id: Some(String::from("UUID")),
            name: Some(String::from("Integration Name")),
            description: Some(String::from("Integration Description")),
            scope: Some(String::from(
                "PUBLIC_CHANNELS | OWNER_ACCESS | CHANNEL_LIST",
            )),
            channel_ids: Some(vec![
                String::from("CHANNEL-A ID for CHANNEL_LIST scope"),
                String::from("CHANNEL-B ID for CHANNEL_LIST scope"),
                String::from("CHANNEL-C ID for CHANNEL_LIST scope"),
            ]),
            headers: Some(vec![
                Header {
                    name: Some(String::from("HeaderName")),
                    value: Some(String::from("HeaderValue")),
                },
                Header {
                    name: Some(String::from("x-my-api-key")),
                    value: Some(String::from("ABC123")),
                },
            ]),
        }
    }
}

/// The headers to include on integration calls
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Header {
    /// The portion in the name part of the header
    pub name: Option<String>,

    /// The portion in the value part of the header
    pub value: Option<String>,
}

/// An event subscription enables a Buzz integration to receive events from Buzz.
/// Event types include the following:
/// * MESSAGE_POSTED A user posted a message
/// * SLASH_COMMAND A user invoked a specific slash command
/// * THREAD_CREATED A thread was created on the parent channel
/// * USERS_JOINED_CHANNEL One or more users joined the channel
/// * USERS_LEFT_CHANNEL One or more users left the channel
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Subscription {
    /// The subscription id
    pub id: Option<String>,

    /// The type of the event. Can be one of the following:
    /// * MESSAGE_POSTED
    /// * SLASH_COMMAND
    /// * THREAD_CREATED
    /// * USERS_JOINED_CHANNEL
    /// * USERS_LEFT_CHANNEL
    pub event_type: Option<String>,

    /// The integration will post to this URL when an event occurs
    pub url: Option<String>,

    /// Required if and only if eventType is SLASH_COMMAND
    pub slash_command: Option<String>,
}

impl Subscription {
    pub fn new() -> Self {
        Subscription {
            id: None,
            event_type: None,
            url: None,
            slash_command: None,
        }
    }

    pub fn template() -> Self {
        Subscription{
            id: Some(String::from("UUID")),
            event_type: Some(String::from("MESSAGE_POSTED | SLASH_COMMAND | THREAD_CREATED | USERS_JOINED_CHANNEL | USERS_LEFT_CHANNEL")),
            url: Some(String::from("The integration will post to this URL when an event occurs")),
            slash_command: Some(String::from("Required if and only if eventType is SLASH_COMMAND")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Event {
    /// Included in events associated with a specific message, this object contains information about the author of the message.
    pub author: Option<User>,

    /// Included in events associated with a thread. This contains information about the thread
    pub thread: Option<Channel>,

    /// Also included in events associated with a specific message, this object contains information about the message itself.
    pub message: Option<Message>,

    /// A list of objects representing users. Only included in USERS_JOINED_CHANNEL and USERS_LEFT_CHANNEL events.
    pub users: Option<Vec<User>>,

    /// Details about the event itself.
    pub event: Option<EventT>,

    /// The user that created the integration.
    pub owner: Option<User>,

    /// Organization represents the Domo customer hosting buzz
    pub organization: Option<Organization>,

    /// The channel in which the event occurred.
    pub channel: Option<Channel>,

    /// URL and headers that the integration may use to post a message back to Buzz. Expires one hour after the event occurred.
    pub callback: Option<Callback>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct EventT {
    /// One of the subscription types
    #[serde(rename = "type")]
    pub event_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Organization {
    pub domain: Option<String>,
}

/// A buzz user object. Simple user
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    /// A user Id
    pub id: Option<u64>,

    /// The users display name
    pub display_name: Option<String>,

    /// The users email address
    pub email: Option<String>,
}

/// A buzz message
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Message {
    /// The id of the buzz message
    pub id: Option<String>,

    /// The text of the buzz message
    pub text: Option<String>,
}

/// A buzz channel
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Channel {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub title: Option<String>,
}

/// Callback information for posting a response message
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Callback {
    pub url: Option<String>,
    pub headers: HashMap<String, String>,
}

/// Buzz Integration API methods
/// Uses the form method_object
impl super::Client {
    /// This endpoint returns all integrations that are active on any channel that the current user has access to.
    pub async fn get_integrations(
        &self,
    ) -> Result<Vec<Integration>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;

        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default, rename_all = "camelCase")]
        struct Ret {
            integrations: Vec<Integration>,
        }
        let mut response = surf::get(&format!("{}{}", self.host, "/v1/buzz/integrations"))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        let ret: Ret = response.body_json().await?;
        Ok(ret.integrations)
    }

    /// Create a new integration
    pub async fn post_integration(
        &self,
        integration: Integration,
    ) -> Result<Integration, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;
        let mut response = surf::post(&format!("{}{}", self.host, "/v1/buzz/integrations"))
            .header("Authorization", at)
            .body(surf::Body::from_json(&integration)?)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieves an integration
    pub async fn get_integration(
        &self,
        id: &str,
    ) -> Result<Integration, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;
        let mut response = surf::get(&format!("{}{}{}", self.host, "/v1/buzz/integrations/", id))
            .header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Permanently deletes a user from your Domo instance
    /// This is destructive and cannot be reversed.
    pub async fn delete_integration(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;
        let mut response =
            surf::delete(&format!("{}{}{}", self.host, "/v1/buzz/integrations/", id))
                .header("Authorization", at)
                .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Get event subscriptions
    pub async fn get_integration_subscriptions(
        &self,
        id: &str,
    ) -> Result<Vec<Subscription>, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;

        #[derive(Serialize, Deserialize, Debug, Default)]
        #[serde(default, rename_all = "camelCase")]
        struct Ret {
            subscriptions: Vec<Subscription>,
        }

        let mut response = surf::get(&format!(
            "{}{}{}{}",
            self.host, "/v1/buzz/integrations/", id, "/subscriptions"
        ))
        .header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        let ret: Ret = response.body_json().await?;
        Ok(ret.subscriptions)
    }

    /// An event subscription enables a Buzz integration to receive events from Buzz. Event types include the following:
    /// * MESSAGE_POSTED: A user posted a message
    /// * SLASH_COMMAND: A user invoked a specific slash command
    /// * THREAD_CREATED: A thread was created on the parent channel
    /// * USERS_JOINED_CHANNEL: One or more users joined the channel
    /// * USERS_LEFT_CHANNEL: One or more users left the channel
    pub async fn post_integration_subscription(
        &self,
        id: &str,
        subscription: Subscription,
    ) -> Result<Subscription, Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;
        let mut response = surf::post(&format!(
            "{}{}{}{}",
            self.host, "/v1/buzz/integrations/", id, "/subscriptions"
        ))
        .header("Authorization", at)
        .body(surf::Body::from_json(&subscription)?)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Deletes an existing event subscription from a Buzz integration. The integration will no longer receive events for the given subscription.
    /// This is destructive and cannot be reversed.
    pub async fn delete_integration_subscription(
        &self,
        id: &str,
        subscription_id: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let at = self.get_access_token("buzz").await?;
        let mut response = surf::delete(&format!(
            "{}{}{}{}{}",
            self.host, "/v1/buzz/integrations/", id, "/subscriptions/", subscription_id
        ))
        .header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }
}
