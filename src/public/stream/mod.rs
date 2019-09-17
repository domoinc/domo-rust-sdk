use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::public::dataset::DataSet;

/// The Stream API allows you to automate the creation of new DataSets in your Domo Warehouse, featuring an accelerated upload Stream. A Domo Stream expedites uploads by dividing your data into parts, and uploading all of these parts simultaneously.
///
/// This API should be used to create and update massive, constantly changing, or rapidly growing DataSets. For creating and updating smaller DataSets that occasionally need data updated, leverage the DataSet API.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Stream {
    /// ID of the Stream
    pub id: Option<u32>,

    /// An ISO-8601 representation of the create date of the Stream
    pub created_at: Option<DateTime<Utc>>,

    /// An ISO-8601 representation of the time the Stream was last updated
    pub modified_at: Option<DateTime<Utc>>,

    /// The data import behavior
    /// The data import behavior: "APPEND", "REPLACE", or "UPSERT".
    pub update_method: Option<String>,

    /// Undocumented, but for upsert update method, defines the key column that pins the changes
    pub key_column_name: Option<String>,

    /// The associated dataset
    #[serde(rename = "dataSet")]
    pub dataset: Option<DataSet>,

    /// If the Stream ID is related to a DataSet that has been deleted, a subset of the Stream's information will be returned, including a deleted property, which will be true
    pub deleted: Option<bool>,
}

impl Stream {
    pub fn new() -> Self {
        Stream {
            id: None,
            created_at: None,
            modified_at: None,
            update_method: None,
            key_column_name: None,
            dataset: None,
            deleted: None,
        }
    }

    pub fn template() -> Self {
        Stream {
            id: Some(0),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            update_method: Some(String::from("APPEND | REPLACE | UPSERT")),
            key_column_name: Some(String::from(
                "Defines the key column used for UPSERT updates",
            )),
            dataset: Some(DataSet::template()),
            deleted: Some(false),
        }
    }
}

/// Defines a stream execution
/// Each new set of data will be a new execution.
/// When committed will trigger Domo to pull in the data from the stream and process it into the
/// existing dataset.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Execution {
    /// Numberic id of execution
    pub id: Option<u32>,

    /// Date Time that this execution was started
    pub started_at: Option<DateTime<Utc>>,

    /// The current state of the exectuion
    pub current_state: Option<String>,

    /// Date Time that this execution was created
    pub created_at: Option<DateTime<Utc>>,

    /// Date Time that this execution was last modified
    pub modified_at: Option<DateTime<Utc>>,
}

/// Stream API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of all Streams for which the user has view permissions.
    ///
    /// Limit: The amount of Stream to return in the list. The default is 50 and the maximum is 500.
    /// Offset: The offset of the Stream ID to begin list of users within the response.
    pub fn get_streams(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Stream>, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/streams"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Returns all Stream objects that meet argument criteria from original request.
    /// This search will return all streams with a matching dataset.id
    pub fn get_stream_search_dataset_id(&self, dsid: &str) -> Result<Vec<Stream>, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/streams/search"))
            .query(&[("q", String::from("dataSource.id:") + dsid)])
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Returns all Stream objects that meet argument criteria from original request.
    /// This search will return all streams with a matching dataset.owner.id
    pub fn get_stream_search_dataset_owner_id(
        &self,
        dsoid: &str,
    ) -> Result<Vec<Stream>, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/streams/search"))
            .query(&[("q", String::from("dataSource.owner.id:") + dsoid)])
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// When creating a Stream, specify the DataSet properties (name and description) and as a convenience, the create Stream API will create a DataSet for you.
    /// In addition, you can only have one Stream open at a time. If you need to add additional data, we recommended adding more parts to the currently open Stream or executing a commit of the open stream before creating a new stream.
    /// The StreamAPI currently only allows you to import data to a DataSet created via the Stream API. For example, it is currently not supported to import data to a DataSet created by a Domo Connector.
    pub fn post_stream(&self, stream: Stream) -> Result<Stream, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/streams"))
            .header("Authorization", at)
            .json(&stream)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an existing stream
    pub fn get_stream(&self, id: &str) -> Result<Stream, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .get(&format!("{}{}{}", self.host, "/v1/streams/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Updates the specified Stream’s metadata by providing values to parameters passed.
    pub fn patch_stream(&self, id: &str, stream: Stream) -> Result<Stream, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .patch(&format!("{}{}{}", self.host, "/v1/streams/", id))
            .header("Authorization", at)
            .json(&stream)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Deletes a Stream from your Domo instance. This does not a delete the associated DataSet.
    pub fn delete_stream(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        self.client
            .delete(&format!("{}{}{}", self.host, "/v1/streams/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Import data into a DataSet in your Domo instance. This request will replace the data currently in the DataSet.
    pub fn get_stream_execution(
        &self,
        id: &str,
        execution_id: &str,
    ) -> Result<Execution, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/streams/", id, "/executions/", execution_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// When you’re ready to upload data to your DataSet via a Stream, you first tell Domo that you’re ready to start sending data by creating an Execution.
    /// Creating an Execution on a Stream will abort all other Executions on that Stream. Each Stream can only have one active Execution at a time.
    pub fn post_stream_execution(&self, id: &str) -> Result<Execution, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .post(&format!(
                "{}{}{}{}",
                self.host, "/v1/streams/", id, "/executions"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Returns all Stream Execution objects that meet argument criteria from original request.
    ///
    /// Limit: The amount of Stream to return in the list. The default is 50 and the maximum is 500
    /// Offset: The offset of the Stream ID to begin list of users within the response
    pub fn get_stream_executions(
        &self,
        id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Execution>, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
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
                self.host, "/v1/streams/", id, "/executions"
            ))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Creates a data part within the Stream execution to upload chunks of rows to the DataSet. The calling client should keep track of parts and order them accordingly in an increasing sequence. If a part upload fails, retry the upload as all parts must be present before committing the stream execution.
    /// Parts can be uploaded simultaneously in separate threads assuming that each part has a distinct part ID and is ordered correctly. To reduce upload time, compress each data as a gzip file (application/gzip)
    /// The only supported content type is currently CSV format.
    /// To upload data in CSV format, the Domo specification used for representing data grids in CSV format closely follows the RFC standard for CSV (RFC-4180)
    pub fn put_stream_execution_part(
        &self,
        id: &str,
        execution_id: &str,
        part_id: &str,
        csv: String,
    ) -> Result<Execution, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .put(&format!(
                "{}{}{}{}{}{}{}",
                self.host, "/v1/streams/", id, "/executions/", execution_id, "/part/", part_id
            ))
            .header("Authorization", at)
            .header("Content-Type", "text/csv")
            .body(csv)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Commits stream execution to import combined set of data parts that have been successfully uploaded.
    /// The Stream API only supports the ability to execute a “commit” every 15 minutes.
    pub fn put_stream_execution_commit(
        &self,
        id: &str,
        execution_id: &str,
    ) -> Result<Execution, Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        Ok(self
            .client
            .put(&format!(
                "{}{}{}{}{}{}",
                self.host, "/v1/streams/", id, "/executions/", execution_id, "/commit"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// If needed during an execution, aborts an entire Stream execution.
    /// To abort the current stream execution within a Stream, simply identify the Stream’s ID within request.
    pub fn put_stream_execution_abort(
        &self,
        id: &str,
        execution_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("data")?;
        self.client
            .put(&format!(
                "{}{}{}{}{}{}",
                self.host, "/v1/streams/", id, "/executions/", execution_id, "/abort"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
