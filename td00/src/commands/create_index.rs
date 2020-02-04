/*
use std::env;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::{BufRead, BufReader};

use std::process;
use std::string::String;

use crossbeam::crossbeam_channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};
*/


use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

//use tantivy::schema::Field;
use tantivy::schema::*;
use tantivy::{Index};

pub fn run_create_index_cli() -> Result<(), String> {
    create_index()
    .map_err(|e| format!("Indexing failed : {:?}", e))
}

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_u64_field("id", FAST | STORED);
    schema_builder.build()
}

fn create_index() -> tantivy::Result<()> {
    let schema = create_schema();

    let check_path = Path::new("/tmp/tantivy/idxbs");
    let dir_exists = check_path.exists();
    if dir_exists {
        remove_dir_all(check_path).expect("dir does not exist");
    }

    let index_path = Path::new("/tmp/tantivy/idxbs");
    create_dir(index_path).expect("dir already exists");

    let _index = Index::create_in_dir(&index_path, schema.clone())?;

    Ok(())
}
