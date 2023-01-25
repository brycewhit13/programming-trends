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

    // print the results
    //println!("{}", json!(results));
    println!("{}", format_result(json!(results)));
}

fn format_result(result: Value) -> String {
    // Access the state and value data
    let states = &result["default"]["geoMapData"][0]["geoName"];
    let num_searches = &result["default"]["geoMapData"][0]["value"][0];
    
    // return formatted results
    format!("{}: {}", states.to_string(), num_searches.to_string())
}