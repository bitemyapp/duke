extern crate duke;

extern crate serde;
extern crate serde_json;

// https://rustbyexample.com/hello/print.html
// fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
use duke::{Boost, Query};

#[test]
fn test_query_match_all() {
    // let query = Query::MatchAllQuery(Some(Boost(1.0)));
    // let query = Query::MatchAllQuery(Some(Boost { boost: 1.0 }));
    let query = Query::MatchAll(duke::MatchAllQuery { boost: None });

    let jstr = serde_json::to_string(&query);
    println!("{:?}", jstr);
}
