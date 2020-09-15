use domo::public::Client;

use structopt::StructOpt;

use super::util;

/// Wraps the activity api
#[derive(StructOpt, Debug)]
pub enum ActivityCommand {
    /// Retrieves activity log entries
    #[structopt(name = "list")]
    List {
        start: u64,
        #[structopt(short = "e", long = "end")]
        end: Option<u64>,
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
        #[structopt(short = "u", long = "user")]
        user_id: Option<u64>,
    },
}

pub async fn execute(dc: Client, template: Option<String>, command: ActivityCommand) {
    match command {
        ActivityCommand::List {
            user_id,
            start,
            end,
            limit,
            offset,
        } => {
            let r = dc
                .get_entries(user_id, start, end, limit, offset)
                .await
                .unwrap();
            util::vec_obj_template_output(r, template);
        }
    }
}
