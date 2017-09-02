use duke::*;

use common::*;

pub fn run_tests() {
    search_all();
    search_all_boosted();
}

fn search_all() {
    let query = Search {
        query: Some(Query::MatchAll(MatchAllQuery {
            boost: None,
        })),
    };
    let res: SearchResponse<Tweet> =
        search(&build_url(""), &INDEX_NAME.to_string(), &query).unwrap();
    assert_eq!(1, res.hits.hits.len());
    assert_eq!(example_tweet(), res.hits.hits[0]._source);
}

fn search_all_boosted() {
    let query = Search {
        query: Some(Query::MatchAll(MatchAllQuery {
            boost: Some(Boost(1.5)),
        })),
    };
    let res: SearchResponse<Tweet> =
        search(&build_url(""), &INDEX_NAME.to_string(), &query).unwrap();
    assert_eq!(1, res.hits.hits.len());
    assert_eq!(example_tweet(), res.hits.hits[0]._source);
}
