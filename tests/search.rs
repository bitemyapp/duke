use duke::*;

use common::*;

pub fn run_tests() {
    search_all();
    search_all_boosted();
    search_term_query();
}

fn search_all() {
    let query = Search {
        query: Some(Query::MatchAll(MatchAllQuery {
            boost: None,
        })),
    };
    let res: SearchResponse<Tweet> =
        search(&build_url(""), INDEX_NAME, &query).unwrap();
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
        search(&build_url(""), INDEX_NAME, &query).unwrap();
    assert_eq!(1, res.hits.hits.len());
    assert_eq!(example_tweet(), res.hits.hits[0]._source);
}


fn search_term_query() {
    let query = Search {
        query: Some(Query::TermQuery(TermQuery {
            term_query_term: Term { term_field: "user_name".to_string(),
                                    term_value: "bitemyapp".to_string() },
            term_query_boost: None,
        })),
    };
    let res: SearchResponse<Tweet> =
        search(&build_url(""), INDEX_NAME, &query).unwrap();
    // let res: Result<SearchResponse<Tweet>, serde_json::error::Error> =
    //     search(&build_url(""), &INDEX_NAME.to_string(), &query);
    // println!("{:?}", res);
    assert_eq!(1, res.hits.hits.len());
    assert_eq!(example_tweet(), res.hits.hits[0]._source);
}
