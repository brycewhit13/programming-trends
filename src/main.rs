// import crates and functions from lib.rs
//use clap::Parser;
//use programming_trends::print_result;
//use programming_trends::plot_result;

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

// TODO: Option to run in python, rust, java mode and get full results

pub fn main() {
    programming_trends::get_all_language_popularity();
}
