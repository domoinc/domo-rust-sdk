use domo::public::page::Collection;
use domo::public::page::Page;
use domo::public::Client;

use structopt::StructOpt;

use super::util;

/// Wraps the page api
#[derive(StructOpt, Debug)]
pub enum PageCommand {
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },
    #[structopt(name = "create")]
    Create {},

    #[structopt(name = "retrieve")]
    Retrieve { id: u64 },

    #[structopt(name = "update")]
    Update { id: u64 },

    #[structopt(name = "delete")]
    Delete { id: u64 },

    #[structopt(name = "list-collections")]
    ListCollections { id: u64 },

    #[structopt(name = "create-collection")]
    CreateCollection { id: u64 },

    #[structopt(name = "update-collection")]
    UpdateCollection { id: u64, collection_id: u64 },

    #[structopt(name = "delete-collection")]
    DeleteCollection { id: u64, collection_id: u64 },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: PageCommand) {
    match command {
        PageCommand::List { limit, offset } => {
            let r = dc.get_pages(limit, offset).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        PageCommand::Create {} => {
            let r = Page::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_page(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        PageCommand::Retrieve { id } => {
            let r = dc.get_page(id).await.unwrap();
            util::obj_template_output(r, template);
        }
        PageCommand::Update { id } => {
            let r = dc.get_page(id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.put_page(id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        PageCommand::Delete { id } => {
            dc.delete_page(id).await.unwrap();
        }
        PageCommand::ListCollections { id } => {
            let r = dc.get_page_collections(id).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        PageCommand::CreateCollection { id } => {
            let r = Collection::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_page_collection(id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        PageCommand::UpdateCollection { id, collection_id } => {
            let r = dc.get_page_collections(id).await.unwrap();
            let r: Collection = {
                let mut ret: Option<Collection> = None;
                for c in r {
                    if let Some(id) = c.id {
                        if id == collection_id {
                            ret = Some(c);
                        }
                    }
                }
                if let Some(c) = ret {
                    c
                } else {
                    panic!("Invalid Collection Id");
                }
            };
            let r = util::edit_obj(editor, r, "").unwrap();
            dc.put_page_collection(id, collection_id, r).await.unwrap();
        }
        PageCommand::DeleteCollection { id, collection_id } => {
            dc.delete_page_collection(id, collection_id).await.unwrap();
        }
    }
}
