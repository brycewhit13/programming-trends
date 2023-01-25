/* Logic for interacting with the rtrend API */

// Import crates
use rtrend::{Client, Country, Keywords, RegionInterest};
use serde_json::{json, Value};

pub fn get_programming_lang_comparison() {
    // vector of programming languages
    let program_langs = vec!["Python"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();

    // print and format the results
    format_result(json!(results));
}

fn format_result(result: Value) {
    // Loop through each state - 51 because DC is included
    for st in 0..51 {
        // Access the state and value data
        let state = &result["default"]["geoMapData"][st]["geoName"];
        let num_searches = &result["default"]["geoMapData"][st]["value"][0];

        // return formatted results
        println!("{}: {}", state, num_searches);
    }
}
