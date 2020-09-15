use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

/// The DataSet object allows you to create, import, export and manage DataSets and manage data permissions for DataSets within Domo.
///
/// The DataSet API should be used to create and update small DataSets that occasionally need their data updated. For creating and updating massive, constantly changing, or rapidly growing DataSets, the Stream API is recommended.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct DataSet {
    /// ID of the DataSet
    pub id: Option<String>,

    /// Name of the DataSet
    pub name: Option<String>,

    /// Description of DataSet
    pub description: Option<String>,

    /// Dataset owner
    pub owner: Option<Owner>,

    /// An ISO-8601 representation of the creation date of the DataSet
    pub created_at: Option<DateTime<Utc>>,

    /// AN ISO-8601 representation of the time the DataSet was last updated
    pub updated_at: Option<DateTime<Utc>>,

    /// An ISO-8601 representation of the time the DataSet was current
    pub data_current_at: Option<DateTime<Utc>>,

    /// The current schema associated with this DataSet
    pub schema: Option<Schema>,

    /// Indicates if PDP [Personalized Data Permission] policy filtering on data is active on this DataSet
    pub pdp_enabled: Option<bool>,

    /// List of policies attached to DataSet
    pub policies: Option<Vec<Policy>>,

    /// The number of rows currently in the DataSet
    pub rows: Option<u64>,

    /// The number of columns currently in the DataSet
    pub columns: Option<u32>,
}

impl DataSet {
    pub fn new() -> Self {
        DataSet {
            id: None,
            name: None,
            description: None,
            owner: None,
            created_at: None,
            updated_at: None,
            data_current_at: None,
            schema: None,
            pdp_enabled: None,
            policies: None,
            rows: None,
            columns: None,
        }
    }
    pub fn template() -> Self {
        DataSet {
            id: Some(String::from("UUID")),
            name: Some(String::from("DataSet Name")),
            description: Some(String::from("DataSet Description")),
            owner: Some(Owner {
                id: 1234,
                name: Some(String::from("DataSet Owner's Name")),
            }),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            data_current_at: Some(Utc::now()),
            schema: Some(Schema {
                columns: Some(vec![Column {
                    name: Some(String::from("Column Name")),
                    column_type: Some(String::from(
                        "STRING | DECIMAL | LONG | DOUBLE | DATE | DATETIME",
                    )),
                }]),
            }),
            pdp_enabled: Some(false),
            policies: Some(vec![Policy::template()]),
            rows: Some(0),
            columns: Some(0),
        }
    }
}

/// The dataset owner
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Owner {
    /// ID of the owner
    pub id: u32,
    /// Name of the owner
    pub name: Option<String>,
}

/// Defines a schema of a dataset
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Schema {
    /// Array of columns in the DataSet
    pub columns: Option<Vec<Column>>,
}

/// A single column schema definition for a dataset
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Column {
    /// Column name in the DataSet schema
    pub name: Option<String>,

    /// Column type in the DataSet schema. Valid types are STRING, DECIMAL, LONG, DOUBLE, DATE, DATETIME.
    #[serde(rename = "type")]
    pub column_type: Option<String>,
}

/// A PDP Policy definition for a dataset
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Policy {
    /// ID of the Policy
    pub id: Option<u32>,

    /// Name of the Policy
    pub name: Option<String>,

    /// Type of policy (user or system)
    #[serde(rename = "type")]
    pub policy_type: Option<String>,

    /// List of filters
    pub filters: Option<Vec<Filter>>,

    /// List of user IDs the policy applies to
    pub users: Option<Vec<u64>>,

    /// List of group IDs the policy applies to
    pub groups: Option<Vec<String>>,
}

impl Policy {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            policy_type: None,
            filters: None,
            users: None,
            groups: None,
        }
    }

    pub fn template() -> Self {
        Self {
            id: Some(0),
            name: Some(String::from("Policy Name")),
            policy_type: Some(String::from("user | system")),
            filters: Some(vec![Filter {
                column: Some(String::from("Column to filter on")),
                not: Some(false),
                operator: Some(String::from("EQUALS")),
                values: vec![String::from("values in this column that match will apply")],
            }]),
            users: Some(vec![27]),
            groups: Some(vec![String::from("15")]),
        }
    }
}

/// Policy filter for a dataset
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Filter {
    /// Name of the column to filter on
    pub column: Option<String>,

    /// Determines if NOT is applied to the filter operation
    pub not: Option<bool>,

    /// Matching operator (EQUALS)
    pub operator: Option<String>,

    /// Values to filter on
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Contains the results from a dataset query
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct QueryResult {
    /// The associated dataset
    /// TODO This should be dataset (consistency)
    pub datasource: Option<String>,

    /// The result set column names
    pub columns: Option<Vec<String>>,

    /// Metadata about the resultset rows and columns
    pub metadata: Option<Vec<QueryMetadata>>,

    /// The raw row data from the result set
    pub rows: Option<Vec<Vec<Value>>>,

    /// The number of rows in the result set
    pub num_rows: Option<u64>,

    /// The number of columns in the result set
    pub num_columns: Option<u32>,

    /// Whether this result set was served from cache
    pub from_cache: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct QueryMetadata {
    /// The column type of the result set, if the column has this defined
    #[serde(rename = "type")]
    pub column_type: Option<String>,

    /// The dataset id
    /// TODO This should be dataset (consistency)
    pub datasource_id: Option<String>,

    /// TODO Not sure what this is
    pub max_length: Option<i32>,

    /// TODO Not sure what this is
    pub min_length: Option<i32>,

    /// TODO Not sure what this is
    pub period_index: Option<u32>,
}

/// DataSet API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of all DataSets in your Domo instance.
    pub async fn get_datasets(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<DataSet>, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        let mut response = surf::get(&format!("{}{}", self.host, "/v1/datasets"))
            .set_query(&q)?
            .set_header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Creates a new DataSet in your Domo instance. Once the DataSet has been created, data can then be imported into the DataSet.
    pub async fn post_dataset(&self, ds: DataSet) -> Result<DataSet, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::post(&format!("{}{}", self.host, "/v1/datasets"))
            .set_header("Authorization", at)
            .body_json(&ds)?
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieves the details of an existing DataSet.
    pub async fn get_dataset(&self, id: &str) -> Result<DataSet, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::get(&format!("{}{}{}", self.host, "/v1/datasets/", id))
            .set_header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Updates the specified DataSet’s metadata by providing values to parameters passed.
    pub async fn put_dataset(&self, id: &str, ds: DataSet) -> Result<DataSet, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::put(&format!("{}{}{}", self.host, "/v1/datasets/", id))
            .set_header("Authorization", at)
            .body_json(&ds)?
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Permanently deletes a DataSet from your Domo instance. This can be done for all DataSets, not just those created through the API.
    ///
    /// This is destructive and cannot be reversed.
    pub async fn delete_dataset(&self, id: &str) -> Result<(), surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::delete(&format!("{}{}{}", self.host, "/v1/datasets/", id))
            .set_header("Authorization", at)
            .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Export data from a DataSet in your Domo instance.
    ///
    /// Data types will be exported as they are currently stored in the dataset. In addition, the only supported export type is CSV.
    ///
    /// TODO Parameters includeHeader and fileName
    pub async fn get_dataset_data(&self, id: &str) -> Result<String, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::get(&format!(
            "{}{}{}{}",
            self.host, "/v1/datasets/", id, "/data"
        ))
        .set_query(&[("includeHeader", "true")])?
        .set_header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_string().await?)
    }

    /// Import data into a DataSet in your Domo instance. This request will replace the data currently in the DataSet.
    ///
    /// The only supported content type is currently CSV format.
    ///
    /// To upload data in CSV format, the Domo specification used for representing data grids in CSV format closely follows the RFC standard for CSV (RFC-4180).
    pub async fn put_dataset_data(&self, id: &str, csv: String) -> Result<(), surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::put(&format!(
            "{}{}{}{}",
            self.host, "/v1/datasets/", id, "/data"
        ))
        .set_header("Authorization", at)
        .set_header("Content-Type", "text/csv")
        .body_string(csv)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Returns data from the DataSet based on your SQL query.
    pub async fn post_dataset_query(
        &self,
        id: &str,
        query: &str,
    ) -> Result<QueryResult, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::post(&format!(
            "{}{}{}",
            self.host, "/v1/datasets/query/execute/", id
        ))
        .set_header("Authorization", at)
        .body_json(&json!({ "sql": query }))?
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// List the Personalized Data Permission (PDP) policies for a specified DataSet.
    pub async fn get_dataset_policies(&self, id: &str) -> Result<Vec<Policy>, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::get(&format!(
            "{}{}{}{}",
            self.host, "/v1/datasets/", id, "/policies"
        ))
        .set_header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Create a PDP policy for user and or group access to data within a DataSet.
    /// Users and groups must exist before creating PDP policy.
    pub async fn post_dataset_policy(
        &self,
        id: &str,
        policy: Policy,
    ) -> Result<Policy, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::post(&format!(
            "{}{}{}{}",
            self.host, "/v1/datasets/", id, "/policies"
        ))
        .set_header("Authorization", at)
        .body_json(&policy)?
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Retrieve a policy from a DataSet within Domo.
    /// A DataSet is required for a PDP policy to exist.
    pub async fn get_dataset_policy(
        &self,
        id: &str,
        policy_id: u32,
    ) -> Result<Policy, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::get(&format!(
            "{}{}{}{}{}",
            self.host, "/v1/datasets/", id, "/policies/", policy_id
        ))
        .set_header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Update the specific PDP policy for a DataSet by providing values to parameters passed.
    pub async fn put_dataset_policy(
        &self,
        id: &str,
        policy_id: u32,
        policy: Policy,
    ) -> Result<Policy, surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::put(&format!(
            "{}{}{}{}{}",
            self.host, "/v1/datasets/", id, "/policies/", policy_id
        ))
        .set_header("Authorization", at)
        .body_json(&policy)?
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }

    /// Permanently deletes a PDP policy on a DataSet in your Domo instance.
    ///
    /// This is destructive and cannot be reversed.
    pub async fn delete_dataset_policy(
        &self,
        id: &str,
        policy_id: u32,
    ) -> Result<(), surf::Exception> {
        let at = self.get_access_token("data").await?;
        let mut response = surf::delete(&format!(
            "{}{}{}{}{}",
            self.host, "/v1/datasets/", id, "/policies/", policy_id
        ))
        .set_header("Authorization", at)
        .await?;
        if !response.status().is_success() {
            let e: Box<super::PubAPIError> = response.body_json().await?;
            return Err(e);
        }
        Ok(response.body_json().await?)
    }
}
