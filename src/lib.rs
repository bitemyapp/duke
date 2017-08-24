#![allow(dead_code)]
#![allow(unused_imports)]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate elastic_derive;
extern crate elastic;
#[macro_use] extern crate log;
extern crate env_logger;

use elastic::prelude::*;
use futures::{Future, Stream};
use hyper::header::ContentType;
use hyper::Method;
use hyper::Request;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
use std::str;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Query>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Query {
    #[serde(rename = "match_all")]
    MatchAll(MatchAllQuery)
}

#[derive(Serialize, Deserialize)]
pub struct MatchAllQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<Boost>
}

#[derive(Serialize, Deserialize)]
pub struct Boost(pub f64);

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[serde(rename_all = "snake_case")]
struct Tweet {
    user_name: String,
    message: String
}

fn twitter_index() -> Index<'static> {
    Index::from("duke_twitter_index")
}

#[test]
pub fn test_talk_to_server() {
    env_logger::init().unwrap();
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = ClientBuilder::new().build().unwrap();
    // let query = json!({
    //     "query": {
    //         "query_string": {
    //             "query": "*"
    //         }
    //     }
    // });
    let query = json!({
        "query": {
            "match_all": {}
        }
    });

    let index_name = "duke_twitter_index".to_string();
    let mapping_name = "tweet".to_string();
    let doc_id = "1".to_string();
    // Reset the Twitter index
    delete_index(build_url(""), &index_name);
    create_index(build_url(""), &index_name);

    // client.create_index(twitter_index).send().unwrap();

    client.put_mapping::<Tweet>(twitter_index()).send().unwrap();

    let example_tweet = Tweet { user_name: "bitemyapp".to_string(), message: "The Industrial Revolution and its consequences have been a disaster for the human race. They have greatly increased the life-expectancy of those of us who live in “advanced” countries, but they have destabilized society, have made life unfulfilling, have subjected human beings to indignities, have led to widespread psychological suffering (in the Third World to physical suffering as well) and have inflicted severe damage on the natural world. The continued development of technology will worsen the situation. It will certainly subject human beings to greater indignities and inflict greater damage on the natural world, it will probably lead to greater social disruption and psychological suffering, and it may lead to increased physical suffering even in “advanced” countries.".to_string() };
    insert_document(build_url(""), &index_name, &mapping_name, &doc_id, &example_tweet);

    // Send the request and process the response.
    let res = client
        .search::<Value>()
        .index(twitter_index())
        .body(query.to_string())
        .send()
        .unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }

    println!("{:?}", res);

}

// #[test]
// pub fn test_talk_to_server() {
//     println!("{}", create_index(build_url(""), "twitter".to_string()));
//     let boosted_query_search = Search { query: Some(Query::MatchAll(MatchAllQuery { boost: Some(Boost(1.5)) }))};
//     let s = dispatch_elasticsearch_request(build_url("/_search"), Method::Post, &Some(boosted_query_search));
//     println!("{}", s);
//     println!("{}", delete_index(build_url(""), "twitter".to_string()));

// }

pub fn build_url(pl: &str) -> String {
    format!("http://localhost:9200{}", pl)
}

pub fn dispatch_elasticsearch_request<T>(url: String, method: Method, json_body: &Option<T>) -> String
 where T: Serialize {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let url = url.parse().unwrap();
    // let req = Request::new(Method::Get, url);

    let mut req = Request::new(method, url);
    {
        let mut headers = req.headers_mut();
        headers.set(ContentType::json());
    }
    match *json_body {
        Some(ref body) => req.set_body(serde_json::to_string(&json_body).unwrap()),
        _ => ()
    };
    let mut s = String::new();
    {
        let work = client.request(req).and_then(|res| {
            res.body().for_each(|chunk| {
                s.push_str(str::from_utf8(&*chunk).unwrap());
                futures::future::ok(())
            })
        });
        core.run(work).unwrap();
    }
    // error!("dispatch_elasticsearch_request passed: {:?} {:?}", method, json_body);
    debug!("dispatch_elasticsearch_request got: {}", s);
    s
}

pub fn create_index(url: String, index: &String) -> String {
    let index_url = format!("{}/{}", url, index);
    // dispatch_elasticsearch_request(index_url, Method::Put, &Some(""))
    dispatch_elasticsearch_request(index_url, Method::Put, &None::<String>)
}

pub fn delete_index(url: String, index: &String) -> String {
    let index_url = format!("{}/{}", url, index);
    // dispatch_elasticsearch_request(index_url, Method::Delete, &Some(""))
    dispatch_elasticsearch_request(index_url, Method::Delete, &None::<String>)
}

pub fn insert_document<T>(url: String, index: &String,
                          mapping: &String, doc_id: &String,
                          doc: &T) -> String where T: Serialize {
    let index_url = format!("{}/{}/{}/{}", url, index, mapping, doc_id);
    dispatch_elasticsearch_request(index_url, Method::Post, &Some(doc))
}

pub struct IndexName(pub String);
