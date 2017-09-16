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
    let doc_id = "1";
    delete_index(&build_url(""), INDEX_NAME);
    create_index(&build_url(""), INDEX_NAME);
    insert_document(
        build_url(""),
        INDEX_NAME,
        MAPPING_NAME,
        doc_id,
        &example_tweet(),
    );
    let wait_time = time::Duration::from_secs(1);
    thread::sleep(wait_time);
}

pub fn main() {
    setup();
    search::run_tests();
}
