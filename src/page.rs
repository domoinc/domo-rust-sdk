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

pub fn execute(dc: Client, e: &str, t: Option<String>, command: PageCommand) {
    match command {
        PageCommand::List { limit, offset } => {
            let r = dc.get_pages(limit, offset).unwrap();
            util::vec_obj_template_output(r, t);
        }
        PageCommand::Create {} => {
            let r = Page::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_page(r).unwrap();
            util::obj_template_output(r, t);
        }
        PageCommand::Retrieve { id } => {
            let r = dc.get_page(id).unwrap();
            util::obj_template_output(r, t);
        }
        PageCommand::Update { id } => {
            let r = dc.get_page(id).unwrap();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.put_page(id, r).unwrap();
            util::obj_template_output(r, t);
        }
        PageCommand::Delete { id } => {
            dc.delete_page(id).unwrap();
        }
        PageCommand::ListCollections { id } => {
            let r = dc.get_page_collections(id).unwrap();
            util::vec_obj_template_output(r, t);
        }
        PageCommand::CreateCollection { id } => {
            let r = Collection::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_page_collection(id, r).unwrap();
            util::obj_template_output(r, t);
        }
        PageCommand::UpdateCollection { id, collection_id } => {
            let r = dc.get_page_collections(id).unwrap();
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
            let r = util::edit_obj(e, r, "").unwrap();
            dc.put_page_collection(id, collection_id, r).unwrap();
        }
        PageCommand::DeleteCollection { id, collection_id } => {
            dc.delete_page_collection(id, collection_id).unwrap();
        }
    }
}
