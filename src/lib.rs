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
extern crate log;

use futures::{Future, Stream};
use hyper::header::ContentType;
use hyper::Method;
use hyper::Request;
use serde::Serialize;
use serde_json::Number;
use serde_json::Value;
use std::fmt::Debug;
use std::str;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")] pub query: Option<Query>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Query {
    #[serde(rename = "match_all")] MatchAll(MatchAllQuery),
}

#[derive(Serialize, Deserialize)]
pub struct MatchAllQuery {
    #[serde(skip_serializing_if = "Option::is_none")] pub boost: Option<Boost>,
}

#[derive(Serialize, Deserialize)]
pub struct Boost(pub f64);

pub fn build_url(pl: &str) -> String {
    format!("http://localhost:9200{}", pl)
}

pub fn dispatch_elasticsearch_request<T>(
    url: String,
    method: Method,
    json_body: &Option<T>,
) -> String
where
    T: Serialize,
{
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let url = url.parse().unwrap();
    // println!("{:?}", url);

    let mut req = Request::new(method, url);
    {
        let mut headers = req.headers_mut();
        headers.set(ContentType::json());
    }
    match *json_body {
        Some(ref body) => req.set_body(serde_json::to_string(&json_body).unwrap()),
        _ => (),
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

pub fn create_index(url: &str, index: &str) -> String {
    let index_url = format!("{}/{}", url, index);
    dispatch_elasticsearch_request(index_url, Method::Put, &None::<String>)
}

pub fn delete_index(url: &str, index: &str) -> String {
    let index_url = format!("{}/{}", url, index);
    dispatch_elasticsearch_request(index_url, Method::Delete, &None::<String>)
}

pub fn insert_document<T>(
    url: String,
    index: &String,
    mapping: &String,
    doc_id: &String,
    doc: &T,
) -> String
where
    T: Serialize,
{
    let index_url = format!("{}/{}/{}/{}", url, index, mapping, doc_id);
    dispatch_elasticsearch_request(index_url, Method::Post, &Some(doc))
}

pub fn search<S, D>(url: &str, index: &str, search_body: &S) -> Result<SearchResponse<D>, serde_json::Error>
where
    S: Serialize,
    D: serde::de::DeserializeOwned,
{
    let search_url = format!("{}/{}/_search", url, index);
    let resp = dispatch_elasticsearch_request(search_url, Method::Post, &Some(search_body));
    serde_json::from_str(&resp)
}

pub struct IndexName(pub String);

// $ curl -XPOST 'http://localhost:9200/duke_twitter_index/_search' -d '{}'
// {"took":1,"timed_out":false,"_shards":{"total":5,"successful":5,"failed":0},"hits":{"total":1,"max_score":1.0,"hits":[{"_index":"duke_twitter_index","_type":"tweet","_id":"1","_score":1.0,"_source":{"user_name":"bitemyapp","message":"The Industrial Revolution and its consequences have been a disaster for the human race. They have greatly increased the life-expectancy of those of us who live in “advanced” countries, but they have destabilized society, have made life unfulfilling, have subjected human beings to indignities, have led to widespread psychological suffering (in the Third World to physical suffering as well) and have inflicted severe damage on the natural world. The continued development of technology will worsen the situation. It will certainly subject human beings to greater indignities and inflict greater damage on the natural world, it will probably lead to greater social disruption and psychological suffering, and it may lead to increased physical suffering even in “advanced” countries."}}]}}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T> {
    took: Number,
    timed_out: bool,
    _shards: Shards,
    hits: Hits<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shards {
    total: Number,
    successful: Number,
    failed: Number,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits<T> {
    total: Number,
    max_score: Number,
    hits: Vec<Hits1<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits1<T> {
    _index: String,
    _type: String,
    _id: String,
    _score: Number,
    _source: T,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Source {
//     user_name: String,
//     message: String,
// }
