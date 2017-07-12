#![allow(dead_code)]
#![allow(unused_imports)]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

extern crate serde;

extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use futures::{Future, Stream};
use hyper::header::ContentType;
use hyper::Method;
use hyper::Request;
use serde::Serialize;
use std::str;

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
    s
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Tweet {
    user_name: String,
    message: String
}

pub fn create_index(url: String, index: String) -> String {
    let index_url = format!("{}/{}", url, index);
    dispatch_elasticsearch_request(index_url, Method::Put, &Some(""))
}

pub fn delete_index(url: String, index: String) -> String {
    let index_url = format!("{}/{}", url, index);
    dispatch_elasticsearch_request(index_url, Method::Delete, &Some(""))
}

pub fn insert_document<T>(url: String, index: String, doc: &T) -> String where T: Serialize {
    let index_url = format!("{}/{}", url, index);
    dispatch_elasticsearch_request(index_url, Method::Post, &Some(doc))
}

#[test]
pub fn test_talk_to_server() {
    println!("{}", create_index(build_url(""), "twitter".to_string()));
    let example_tweet = Tweet { user_name: "bitemyapp".to_string(), message: "The Industrial Revolution and its consequences have been a disaster for the human race. They have greatly increased the life-expectancy of those of us who live in “advanced” countries, but they have destabilized society, have made life unfulfilling, have subjected human beings to indignities, have led to widespread psychological suffering (in the Third World to physical suffering as well) and have inflicted severe damage on the natural world. The continued development of technology will worsen the situation. It will certainly subject human beings to greater indignities and inflict greater damage on the natural world, it will probably lead to greater social disruption and psychological suffering, and it may lead to increased physical suffering even in “advanced” countries.".to_string() };
    println!("{}", insert_document(build_url(""), "twitter".to_string(), &example_tweet));
    let boosted_query_search = Search { query: Some(Query::MatchAll(MatchAllQuery { boost: Some(Boost(1.5)) }))};
    let s = dispatch_elasticsearch_request(build_url("/_search"), Method::Post, &Some(boosted_query_search));
    println!("{}", s);
    println!("{}", delete_index(build_url(""), "twitter".to_string()));

}

pub struct IndexName(pub String);

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
