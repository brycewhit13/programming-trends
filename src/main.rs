// import crates
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "4.0.32",
    author = "Bryce Whitney",
    about = "A command-line tool that looks at programming language trends around the country",
    after_help = "Example: cargo run OR cargo run -- single --langauge Rust"
)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Choose a single language to see the full results for. Options are Rust, Python, C, Java, and Javascript")]
    Single {
        #[clap(short, long, default_value = "Rust")]
        language: Option<String>, // Determine what language to look at
    }
}

pub fn main() {
    let args = Args::parse();
    match args.command{
        Some(Commands::Single { language }) => {
            match language.unwrap().as_str() {
                "Rust" => {
                    let result = programming_trends::_get_rust_popularity();
                    programming_trends::print_complete_result(result);
                }
                "Python" => {
                    let result = programming_trends::_get_python_popularity();
                    programming_trends::print_complete_result(result);
                }
                "Java" => {
                    let result = programming_trends::_get_java_popularity();
                    programming_trends::print_complete_result(result);
                }
                "C" => {
                    let result = programming_trends::_get_c_popularity();
                    programming_trends::print_complete_result(result);
                }
                "Javascript" => {
                    let result = programming_trends::_get_javascript_popularity();
                    programming_trends::print_complete_result(result);
                }
                &_ => {
                    let result = programming_trends::_get_rust_popularity();
                    programming_trends::print_complete_result(result);
                }
            }
        }
        _ => {
            programming_trends::get_all_language_popularity();
        }
    }
}
