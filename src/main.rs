use domo::public::Client;

use structopt::StructOpt;

mod account;
mod activity;
mod buzz;
mod dataset;
mod group;
mod page;
mod stream;
mod user;
mod util;
mod wh;
mod workflow;

/// Wraps the sdk and offers a cli application
///
/// To get started go to https://developer.domo.com,
/// Sign-In, and create a client.
#[derive(StructOpt, Debug)]
#[structopt(name = "domo")]
struct DomoApp {
    /// Define a custom editor to use
    #[structopt(long = "editor", default_value = "vim", env = "DOMO_EDITOR")]
    editor: String,

    /// This defines the environment you need to connect to.
    /// The default is api.domo.com and will work for most use cases.
    /// If you are pointing at a test, dev, or demo lane, you many need to change this.
    #[structopt(
        long = "host",
        default_value = "https://api.domo.com",
        env = "DOMO_API_HOST"
    )]
    host: String,

    /// This is your public api client_id.
    #[structopt(long = "clientid", env = "DOMO_API_CLIENT_ID")]
    client_id: String,

    /// This is your public api client_secret.
    #[structopt(long = "clientsecret", env = "DOMO_API_CLIENT_SECRET")]
    client_secret: String,

    /// This application can output in different formats, like json, csv, or yaml.
    /// It will default to yaml where possible as it is easier to read in the terminal.
    /// You can override if you'd like to output a more convient format.
    #[structopt(short = "t", long = "template")]
    /// Defines the output template. Can be json, csv, yaml, and debug. Used if the command supports variable output
    template: Option<String>,

    /// The different apis will be available as subcommands
    #[structopt(subcommand)]
    command: DomoCommand,
}

/// The different apis will be available as subcommands
#[derive(StructOpt, Debug)]
enum DomoCommand {
    /// Wraps the account api
    #[structopt(name = "account")]
    Account {
        #[structopt(subcommand)]
        command: account::AccountCommand,
    },

    /// Wraps the activity api
    #[structopt(name = "activity")]
    Activity {
        #[structopt(subcommand)]
        command: activity::ActivityCommand,
    },

    /// Wraps the buzz api
    #[structopt(name = "buzz")]
    Buzz {
        #[structopt(subcommand)]
        command: buzz::BuzzCommand,
    },

    /// Wraps the dataset api
    #[structopt(name = "dataset")]
    DataSet {
        #[structopt(subcommand)]
        command: dataset::DataSetCommand,
    },

    /// Wraps the group api
    #[structopt(name = "group")]
    Group {
        #[structopt(subcommand)]
        command: group::GroupCommand,
    },

    /// Wraps the page api
    #[structopt(name = "page")]
    Page {
        #[structopt(subcommand)]
        command: page::PageCommand,
    },

    /// Wraps the stream api
    #[structopt(name = "stream")]
    Stream {
        #[structopt(subcommand)]
        command: stream::StreamCommand,
    },

    /// Wraps the user api
    #[structopt(name = "user")]
    User {
        #[structopt(subcommand)]
        command: user::UserCommand,
    },

    /// Wraps domo webhooks
    #[structopt(name = "webhook")]
    Webhook {
        #[structopt(subcommand)]
        command: wh::WebhookCommand,
    },

    /// Wraps the workflow api
    #[structopt(name = "workflow")]
    Workflow {
        #[structopt(subcommand)]
        command: workflow::WorkflowCommand,
    },
}

#[async_std::main]
async fn main() {
    let app = DomoApp::from_args();

    let dc = Client::new(&app.host, &app.client_id, &app.client_secret);

    match app.command {
        DomoCommand::Account { command } => {
            account::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::Activity { command } => activity::execute(dc, app.template, command).await,
        DomoCommand::Buzz { command } => {
            buzz::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::DataSet { command } => {
            dataset::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::Group { command } => {
            group::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::Page { command } => {
            page::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::Stream { command } => {
            stream::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::User { command } => {
            user::execute(dc, &app.editor, app.template, command).await
        }
        DomoCommand::Webhook { command } => wh::execute(&app.editor, command).await,
        DomoCommand::Workflow { command } => {
            workflow::execute(dc, &app.editor, app.template, command).await
        }
    }
}
