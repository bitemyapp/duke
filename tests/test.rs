extern crate duke;

extern crate serde;
#[macro_use]
extern crate serde_json;

use duke::{Boost, Query};

#[test]
fn test_query_match_all_json() {
    let boosted_query = Query::MatchAll(duke::MatchAllQuery { boost: Some(Boost(1.5)) });
    let boosted_query_expect = json!({"match_all": {"boost": 1.5}});
    let no_boost_query = Query::MatchAll(duke::MatchAllQuery { boost: None });
    let no_boost_query_expect = json!({"match_all": {}});

    assert_eq!(serde_json::to_value(boosted_query).unwrap(),
               boosted_query_expect);
    assert_eq!(serde_json::to_value(no_boost_query).unwrap(),
               no_boost_query_expect);
}
