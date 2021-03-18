use domo::public::dataset::QueryResult;

use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

use csv::{ReaderBuilder, Writer};
use serde::Serialize;
use serde_json::Value;

pub fn vec_obj_template_output<T: Serialize + Debug>(r: Vec<T>, template: Option<String>) {
    match template.as_deref() {
        Some("debug") => println!("{:#?}", r),
        Some("json") => {
            println!("{}", serde_json::to_string(&r).unwrap());
        }
        Some("yaml") => {
            println!("{}", serde_yaml::to_string(&r).unwrap());
        }
        Some("csv") => {
            let mut w = Writer::from_writer(io::stdout());
            for o in r {
                w.serialize(o).unwrap();
            }
        }
        _ => println!("{}", serde_yaml::to_string(&r).unwrap()),
    }
}

pub fn obj_template_output<T: Serialize + Debug>(r: T, template: Option<String>) {
    match template.as_deref() {
        Some("debug") => println!("{:#?}", r),
        Some("json") => {
            println!("{}", serde_json::to_string(&r).unwrap());
        }
        Some("yaml") => {
            println!("{}", serde_yaml::to_string(&r).unwrap());
        }
        _ => println!("{}", serde_yaml::to_string(&r).unwrap()),
    }
}

pub fn query_template_output(r: QueryResult, template: Option<String>) {
    match template.as_deref() {
        Some("debug") => println!("{:#?}", r),
        Some("json") => {
            println!("{}", serde_json::to_string(&r).unwrap());
        }
        Some("yaml") => {
            println!("{}", serde_yaml::to_string(&r).unwrap());
        }
        Some("csv") => {
            let mut w = Writer::from_writer(io::stdout());
            w.write_record(r.columns.unwrap()).unwrap();
            for row in r.rows.unwrap() {
                for c in row {
                    match c {
                        Value::Number(field) => w.write_field(field.to_string()),
                        Value::String(field) => w.write_field(field),
                        _ => w.write_field(""),
                    }
                    .unwrap();
                }
                //Write end of record
                w.write_record(None::<&[u8]>).unwrap();
            }
        }
        _ => println!("{}", serde_yaml::to_string(&r).unwrap()),
    }
}

pub fn csv_template_output(r: String, template: Option<String>) {
    match template.as_deref() {
        Some("debug") => println!("{}", r),
        Some("json") => {
            let mut aggr: Vec<Vec<String>> = Vec::new();
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(r.as_bytes());
            while let Some(result) = rdr.records().next() {
                let record = result.unwrap();
                aggr.push(record.iter().map(String::from).collect());
            }
            println!("{}", serde_json::to_string(&aggr).unwrap());
        }
        Some("yaml") => {
            let mut aggr: Vec<Vec<String>> = Vec::new();
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(r.as_bytes());
            while let Some(result) = rdr.records().next() {
                let record = result.unwrap();
                aggr.push(record.iter().map(String::from).collect());
            }
            println!("{}", serde_yaml::to_string(&aggr).unwrap());
        }
        _ => println!("{}", r),
    }
}

pub fn edit_obj<T: Serialize>(editor: &str, obj: T, help: &str) -> Result<T, Box<dyn Error>>
where
    for<'de> T: serde::de::Deserialize<'de>,
{
    //Serialize the object as yaml out to a temporary file
    let mut dir = env::temp_dir();
    dir.push("domo_tmp_edit_obj.yaml");
    let mut contents = serde_yaml::to_string(&obj)?;
    contents.push_str(help);
    {
        let mut f = File::create(&dir)?;
        f.write_all(contents.as_bytes())?;
        f.sync_all()?;
    }

    //Execute the editor command
    let mut editor_cmd = Command::new(editor);
    editor_cmd.arg(&dir);
    editor_cmd.status()?;

    //When it's finished read the contents of the file back in as a string
    let yaml = fs::read_to_string(&dir)?;
    let ret: T = serde_yaml::from_str(&yaml)?;

    Ok(ret)
}

pub fn edit_md(editor: &str, markdown: &str) -> Result<String, Box<dyn Error>> {
    let mut dir = env::temp_dir();
    dir.push("domo_tmp_edit_str.md");

    {
        let mut f = File::create(&dir)?;
        f.write_all(markdown.as_bytes())?;
        f.sync_all()?;
    }

    //Execute the editor command
    let mut editor_cmd = Command::new(editor);
    editor_cmd.arg(&dir);
    editor_cmd.status()?;

    //When it's finished read the contents of the file back in as a string
    Ok(fs::read_to_string(&dir)?)
}
