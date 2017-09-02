#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(custom_attribute)]

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
use serde::Serializer;
use serde::ser::SerializeMap;
use serde_json::Number;
use serde_json::Value;
use std::fmt::Debug;
use std::str;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")] pub query: Option<Query>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Query {
    #[serde(rename = "match_all")] MatchAll(MatchAllQuery),
    #[serde(rename = "term")] TermQuery(TermQuery),
}

// #[derive(Serialize, Deserialize)]
// #[derive(Deserialize)]

pub struct TermQuery {
    pub term_query_term: Term,
    pub term_query_boost: Option<Boost>,
}

pub struct Term {
    pub termField: String,
    pub termValue: String,
}

// {
//           "term": {
//             "status": {
//               "value": "urgent",
//               "boost": 2.0 
//             }
//           }
// }

impl Serialize for TermQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(None)?;
        let mut term_map = serializer.serialize_map(None)?;
        let mut term_value_map = serializer.serialize_map(None)?;
        term_value_map.serialize_entry("value", self.term_query_term.termValue);
        match *self.term_query_boost {
            Some(boost) => term_value_map.serialize_entry("boost", boost),
            _ => (),
        }
        term_map.serialize_entry(&self.term_query_term.termField, &term_value_map);
        map.serialize_entry("term", &term_map);
        map.end()
        // let jsonVal =
        //     json!({"term":
        //            {self.termField:
        //             {"value": self.termValue}}});
        // serializer.serialize(jsonVal)
        // jsonVal.serialize(serializer);
        // serializer
        // let mut map = serializer.serialize_map(None)?;
        // map.serialize_entry(""
        // for (k, v) in self {
        //     map.serialize_entry(k, v)?;
        // }
        // map.end()
    }
}

// TermQuery                     Term (Maybe Boost)
// data Term = Term { termField :: Text
//                  , termValue :: Text } deriving (Eq, Read, Show, Generic, Typeable)

#[derive(Serialize)]
pub struct MatchAllQuery {
    #[serde(skip_serializing_if = "Option::is_none")] pub boost: Option<Boost>,
}

#[derive(Serialize)]
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
        // let mut headers = req.headers_mut();
        let headers = req.headers_mut();
        headers.set(ContentType::json());
    }
    match *json_body {
        Some(_) => req.set_body(serde_json::to_string(&json_body).unwrap()),
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

pub fn search<S, D>(
    url: &str,
    index: &str,
    search_body: &S,
) -> Result<SearchResponse<D>, serde_json::Error>
where
    S: Serialize,
    D: serde::de::DeserializeOwned,
{
    let search_url = format!("{}/{}/_search", url, index);
    let resp = dispatch_elasticsearch_request(search_url, Method::Post, &Some(search_body));
    serde_json::from_str(&resp)
}

pub struct IndexName(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T> {
    pub took: Number,
    pub timed_out: bool,
    pub _shards: Shards,
    pub hits: Hits<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shards {
    pub total: Number,
    pub successful: Number,
    pub failed: Number,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits<T> {
    pub total: Number,
    pub max_score: Number,
    pub hits: Vec<Hits1<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits1<T> {
    pub _index: String,
    pub _type: String,
    pub _id: String,
    pub _score: Number,
    pub _source: T,
}
