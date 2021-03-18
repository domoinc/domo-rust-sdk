use super::util;
use domo::public::stream::Stream;
use domo::public::Client;

use std::path::PathBuf;

use structopt::StructOpt;

/// Wraps the stream api
#[derive(StructOpt, Debug)]
pub enum StreamCommand {
    /// Get a list of Streams for which the user has view permissions.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },

    /// Get a list of all Streams for which the user has view permissions.
    #[structopt(name = "list-all")]
    ListAll {},

    /// Create a new stream
    #[structopt(name = "create")]
    Create {},

    /// Retrieves the details of an existing stream
    #[structopt(name = "retrieve")]
    Retrieve { stream_id: String },

    /// Update a stream
    #[structopt(name = "update")]
    Update { stream_id: String },

    /// Deletes a Stream from your Domo instance. This does not a delete the associated DataSet.
    #[structopt(name = "delete")]
    Delete { stream_id: String },

    /// Search streams by owner id
    #[structopt(name = "search-owners")]
    SearchOwners { owner_id: String },

    /// Search streams by dataset id
    #[structopt(name = "search-ids")]
    SearchDatasetId { dataset_id: String },

    /// Returns Stream Execution objects.
    #[structopt(name = "list-executions")]
    ListExecutions {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
        stream_id: String,
    },

    /// When you’re ready to upload data to your DataSet via a Stream, you first tell Domo that you’re ready to start sending data by creating an Execution.
    #[structopt(name = "create-execution")]
    CreateExecution { stream_id: String },

    /// Retrieves a stream execution by id
    #[structopt(name = "retrieve-execution")]
    RetrieveExecution {
        stream_id: String,
        execution_id: String,
    },

    /// Creates a data part within the Stream execution to upload chunks of rows to the DataSet. The calling client should keep track of parts and order them accordingly in an increasing sequence. If a part upload fails, retry the upload as all parts must be present before committing the stream execution.
    #[structopt(name = "upload-part")]
    UploadPart {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        stream_id: String,
        execution_id: String,
        part_id: String,
    },

    /// Commits stream execution to import combined set of data parts that have been successfully uploaded.
    #[structopt(name = "commit-execution")]
    CommitExecution {
        stream_id: String,
        execution_id: String,
    },

    /// If needed during an execution, aborts an entire Stream execution.
    #[structopt(name = "abort-execution")]
    AbortExecution {
        stream_id: String,
        execution_id: String,
    },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: StreamCommand) {
    match command {
        StreamCommand::List { limit, offset } => {
            let r = dc.get_streams(limit, offset).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        StreamCommand::ListAll {} => {
            let mut offset = 0_u32;
            let mut r: Vec<Stream> = Vec::new();
            loop {
                let mut ret = dc.get_streams(Some(50), Some(offset)).await.unwrap();
                let mut b = false;
                if ret.len() < 50 {
                    b = true;
                }
                //Either way slurp all the elements into the aggregator
                r.append(&mut ret);
                offset += 50;
                if b {
                    break;
                }
            }
            util::vec_obj_template_output(r, template);
        }
        StreamCommand::SearchOwners { owner_id } => {
            let r = dc
                .get_stream_search_dataset_owner_id(&owner_id)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
        StreamCommand::SearchDatasetId { dataset_id } => {
            let r = dc.get_stream_search_dataset_id(&dataset_id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        StreamCommand::Create {} => {
            let r = Stream::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_stream(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::Retrieve { stream_id } => {
            let r = dc.get_stream(&stream_id).await.unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::Update { stream_id } => {
            let r = dc.get_stream(&stream_id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.patch_stream(&stream_id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::Delete { stream_id } => {
            dc.delete_stream(&stream_id).await.unwrap();
        }
        StreamCommand::ListExecutions {
            stream_id,
            limit,
            offset,
        } => {
            let r = dc
                .get_stream_executions(&stream_id, limit, offset)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
        StreamCommand::CreateExecution { stream_id } => {
            let r = dc.post_stream_execution(&stream_id).await.unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::RetrieveExecution {
            stream_id,
            execution_id,
        } => {
            let r = dc
                .get_stream_execution(&stream_id, &execution_id)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::UploadPart {
            file,
            stream_id,
            execution_id,
            part_id,
        } => {
            dc.put_stream_execution_part(&stream_id, &execution_id, &part_id, file)
                .await
                .unwrap();
        }
        StreamCommand::CommitExecution {
            stream_id,
            execution_id,
        } => {
            let r = dc
                .put_stream_execution_commit(&stream_id, &execution_id)
                .await
                .unwrap();
            util::obj_template_output(r, template);
        }
        StreamCommand::AbortExecution {
            stream_id,
            execution_id,
        } => {
            dc.put_stream_execution_abort(&stream_id, &execution_id)
                .await
                .unwrap();
        }
    }
}
