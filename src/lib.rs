/* Logic for interacting with the rtrend API */

// Import crates
use rtrend::{Client, Country, Keywords, RegionInterest};
use serde_json::{Value};
use plotly::{Plot, Bar};
use plotly::common::Title;

pub fn get_rust_popularity() -> Value {
    // vector of programming languages
    let program_langs = vec!["Rust"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();
    
    // Return the results
    results
}


pub fn print_result(result: Value) {
    // Loop through each state - 51 because DC is included
    for st in 0..51 {
        // Access the state and value data
        let state = &result["default"]["geoMapData"][st]["geoName"];
        let num_searches = &result["default"]["geoMapData"][st]["value"][0];

        // return formatted results
        println!("{}: {}", state, num_searches);
    }
}

pub fn plot_result(result: Value) {
    // Get vector of states and values
    let mut states: Vec<Value> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    for st in 0..51 {
        states.push(result["default"]["geoMapData"][st]["geoName"].clone());
        values.push(result["default"]["geoMapData"][st]["value"][0].clone());
    }

    // Plot the results
    let mut plot = Plot::new();
    let trace = Bar::new(states, values);
    plot.add_trace(trace);

    // Show the plot
    plot.write_html("output.html");
}