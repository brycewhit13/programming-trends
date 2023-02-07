/* Logic for interacting with the rtrend API */

// Import crates 
use rtrend::{Client, Country, Keywords, RegionInterest};
use serde_json::{Value};
use plotly::{Plot, Bar};

pub fn get_all_language_popularity(){
    // Get the results for each language
    let rust_results = _get_rust_popularity();
    let python_results = _get_python_popularity();
    let java_results = _get_java_popularity();
    let c_results = _get_c_popularity();
    let js_results = _get_javascript_popularity();

    // Print the top-5 states for each language
    for language in vec![("RUST", rust_results), ("PYTHON", python_results), ("JAVA", java_results), ("C", c_results), ("JAVASCRIPT", js_results)]{
        let lang = language.0;
        let res = language.1;
        let header = format!("Top-5 states where {lang} is most popular:");

        println!("{}", header);
        print_top_5(res.clone());

        // Store the graph results for each in a folder
        let output_name = format!("{lang}_results.html");
        plot_result(res, &output_name);
    }

}

pub fn print_top_5(result: Value) {
    // Loop through each state - 51 because DC is included
    for st in 0..5 {
        // Access the state and value data
        let state = &result["default"]["geoMapData"][st]["geoName"];
        let num_searches = &result["default"]["geoMapData"][st]["value"][0];

        // return formatted results
        println!("{}: {}", state, num_searches);
    }

    // Print a newline at the end
    println!("");
}

pub fn print_complete_result(result: Value){
    // Loop through each state - 51 because DC is included
    for st in 0..51 {
        // Access the state and value data
        let state = &result["default"]["geoMapData"][st]["geoName"];
        let num_searches = &result["default"]["geoMapData"][st]["value"][0];

        // return formatted results
        println!("{}: {}", state, num_searches);
    }

}

pub fn plot_result(result: Value, output_filename: &str) {
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

    // Save the plot into a results
    match std::fs::create_dir_all("results"){
        Err(e) => println!("{}", e),
        _ => ()
    };
    plot.write_html(format!("results/{output_filename}"));
}

///////////////////////
// PRIVATE FUNCTIONS //
///////////////////////

pub fn _get_rust_popularity() -> Value {
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

pub fn _get_python_popularity() -> Value {
    // vector of programming languages
    let program_langs = vec!["Python"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();
    
    // Return the results
    results
}

pub fn _get_java_popularity() -> Value {
    // vector of programming languages
    let program_langs = vec!["Java"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();
    
    // Return the results
    results
}

pub fn _get_c_popularity() -> Value {
    // vector of programming languages
    let program_langs = vec!["C"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();
    
    // Return the results
    results
}

pub fn _get_javascript_popularity() -> Value {
    // vector of programming languages
    let program_langs = vec!["Javascript"];

    // Make the request with rtrend
    let keywords = Keywords::new(program_langs);
    let country = Country::US; // Use searches in the united states

    let client = Client::new(keywords, country).build();

    let results = RegionInterest::new(client).get();
    
    // Return the results
    results
}