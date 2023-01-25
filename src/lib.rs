/* Logic for interacting with the rtrend API */

// Import crates
use rtrend::{Keywords, Country, Client, RegionInterest};
use serde_json::{Value, json};

pub fn get_programming_lang_comparison(){
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
    // Loop through each state
    for st in 0..51{ // 51 because DC is included
        // Access the state and value data
        let state = &result["default"]["geoMapData"][st]["geoName"];
        let num_searches = &result["default"]["geoMapData"][st]["value"][0];
        
        // return formatted results
        println!("{}: {}", state.to_string(), num_searches.to_string());
    }
}