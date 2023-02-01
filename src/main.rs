// import crates and functions from lib.rs
//use clap::Parser;
use serde_json::{json, Value};
use programming_trends::get_rust_popularity;
use programming_trends::print_result;
use programming_trends::plot_result;

/* 
// Add extended help
#[derive(Parser, Debug)]
#[clap(author = "Bryce Whitney", 
        version = "4.1.1", 
        about = "A command-line tool that looks at the popularity of different programming languages", 
        after_help = "Example: cargo run --language [LANGUAGE]")]
struct Cli {
   // The programming language
   #[arg(short, long)]
   language: &str,
} */

pub fn main() {
    //let args = Cli::parse();
    // Get the comparison
    let results: Value = get_rust_popularity();

    // Print the results
    print_result(json!(results));

    // PLot the results
    plot_result(json!(results));

}
