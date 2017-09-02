extern crate duke;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use duke::*;
use std::{thread, time};

mod common;
use common::*;

mod search;

pub fn setup() {
    env_logger::init().unwrap();
    let doc_id = "1".to_string();
    delete_index(&build_url(""), &INDEX_NAME.to_string());
    create_index(&build_url(""), &INDEX_NAME.to_string());
    insert_document(
        build_url(""),
        &INDEX_NAME.to_string(),
        &MAPPING_NAME.to_string(),
        &doc_id,
        &example_tweet(),
    );
    let wait_time = time::Duration::from_secs(1);
    thread::sleep(wait_time);
}

pub fn main() {
    setup();
    search::run_tests();
}
