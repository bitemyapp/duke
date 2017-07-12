extern crate hyper;

extern crate serde_json;

// use serde_json;

#[macro_use]
extern crate serde_derive;

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

// #[derive(Serialize, Deserialize)]
// pub struct MatchAll { };

// {"MatchAllQuery": 1.0}
// {"MatchAllQuery":{"boost":1.0}}"
// {"match_all_query":{"boost":1.0}}

// Goal:
// {"match_all":{"boost":1.0}}
// {"query": {"match_all": {"boost": 1.0}}}

//    #[serde(skip_serializing_if = "Option::is_none")]

#[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Boost(pub f64);
// pub struct Boost {
//     pub boost: f64
// }
