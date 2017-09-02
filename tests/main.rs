extern crate duke;

const index_name: String = "duke_twitter_index".to_string();
const mapping_name: String = "tweet".to_string();

pub fn setup() {
    env_logger::init().unwrap();
    let index_name = "duke_twitter_index".to_string();
    let mapping_name = "tweet".to_string();
    let doc_id = "1".to_string();
    delete_index(build_url(""), &index_name);
    create_index(build_url(""), &index_name);
    let example_tweet = Tweet { user_name: "bitemyapp".to_string(), message: "The Industrial Revolution and its consequences have been a disaster for the human race. They have greatly increased the life-expectancy of those of us who live in “advanced” countries, but they have destabilized society, have made life unfulfilling, have subjected human beings to indignities, have led to widespread psychological suffering (in the Third World to physical suffering as well) and have inflicted severe damage on the natural world. The continued development of technology will worsen the situation. It will certainly subject human beings to greater indignities and inflict greater damage on the natural world, it will probably lead to greater social disruption and psychological suffering, and it may lead to increased physical suffering even in “advanced” countries.".to_string() };
    insert_document(build_url(""),
                    &index_name,
                    &mapping_name,
                    &doc_id,
                    &example_tweet);
    let wait_time = time::Duration::from_secs(1);
    thread::sleep(wait_time);
}

pub fn main() {
    setup();
    let query = Search { query: Some(Query::MatchAll(MatchAllQuery { boost: Some(Boost(1.5)) }))};
    let res = search(build_url(""), &index_name, &query);
    println!("{:?}", res);

}
