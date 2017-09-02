
extern crate duke;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use duke::*;
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Tweet {
    user_name: String,
    message: String,
}

const index_name: &'static str = "duke_twitter_index";
const mapping_name: &'static str = "tweet";

pub fn setup() {
    env_logger::init().unwrap();
    let doc_id = "1".to_string();
    delete_index(&build_url(""), &index_name.to_string());
    create_index(&build_url(""), &index_name.to_string());
    let example_tweet = Tweet { user_name: "bitemyapp".to_string(), message: "The Industrial Revolution and its consequences have been a disaster for the human race. They have greatly increased the life-expectancy of those of us who live in “advanced” countries, but they have destabilized society, have made life unfulfilling, have subjected human beings to indignities, have led to widespread psychological suffering (in the Third World to physical suffering as well) and have inflicted severe damage on the natural world. The continued development of technology will worsen the situation. It will certainly subject human beings to greater indignities and inflict greater damage on the natural world, it will probably lead to greater social disruption and psychological suffering, and it may lead to increased physical suffering even in “advanced” countries.".to_string() };
    insert_document(
        build_url(""),
        &index_name.to_string(),
        &mapping_name.to_string(),
        &doc_id,
        &example_tweet,
    );
    let wait_time = time::Duration::from_secs(1);
    thread::sleep(wait_time);
}

// 
pub fn main() {
    setup();
    let query = Search {
        query: Some(Query::MatchAll(MatchAllQuery {
            boost: Some(Boost(1.5)),
        })),
    };
    let res: Result<SearchResponse<Tweet>, serde_json::Error> = search(&build_url(""), &index_name.to_string(), &query);
    println!("{:?}", res);
}
