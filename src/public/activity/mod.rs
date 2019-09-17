use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Activity Log Entry Object
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct LogEntry {
    /// The name of the user
    pub user_name: Option<String>,

    /// The id of the user
    pub user_id: Option<String>,

    /// Type of user
    pub user_type: Option<String>,

    /// Id of proxying user if applicable
    pub actor_id: Option<u64>,

    /// Type of user performing the proxy if applicable
    pub actor_type: Option<String>,

    /// Name of object being affected
    pub object_name: Option<String>,

    /// The ID of the object affected
    pub object_id: Option<String>,

    /// The type of object affected
    pub object_type: Option<String>,

    /// Description of the log entry, with additional information if applicable
    pub additional_comment: Option<String>,

    /// The time the event took place
    pub time: Option<DateTime<Utc>>,

    /// Text describing the main event of the log entry
    pub event_text: Option<String>,

    /// The device type the event came from
    pub device: Option<String>,

    /// The acting users browser details
    pub browser_details: Option<String>,

    /// The ip address of where the event took place
    pub ip_address: Option<String>,
}

/// Activity Log API methods
/// Uses the form method_object
impl super::Client {
    /// Retrieves activity log entries
    ///
    /// Params
    /// * user_id: The Id of the user
    /// * start: The start time(milliseconds) of when you want to receive log events
    /// * end: The end time(milliseconds) of when you want to receive log events
    /// * limit: The maximum number of events you want to retrieve(default is 50, maximum of 1000)
    /// * offset: The offset location of events you retrieve(default is 0)
    pub fn get_entries(
        &self,
        user_id: Option<u64>,
        start: u64,
        end: Option<u64>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<LogEntry>, Box<dyn Error>> {
        let at = self.get_access_token("audit")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(uid) = user_id {
            q.push(("user", uid.to_string()));
        }
        q.push(("start", start.to_string()));
        if let Some(v) = end {
            q.push(("end", v.to_string()));
        }
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/audit"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }
}
