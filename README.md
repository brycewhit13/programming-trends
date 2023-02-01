# Programming Trends
![worflow status](https://github.com/brycewhit13/rust-miniprojects/actions/workflows/rust.yml/badge.svg)

## Motivation
The goal of this project is to analyze google search trends to determine what is currently hot in the programming world and what may be dying off or up and coming. This project uses [Google Trends](https://trends.google.com/trends/?geo=US) using the [rtrend](https://crates.io/crates/rtrend) package in rust, which is an unnoficial API for interacting with Google Trend.

## Understanding the Output
### Printed Output
Currently the program is looking at the popularity of the Rust programming language in each state, and printing out the results. The largest possible score is 100 and this means that is the state where Rust is most commonly searched (and therefore likely most popular). A score of 50 indicates that Rust is half as popular in that state compared to the state with a score of 100. Finally, a score of 0 does not mean there are not any searches at all, it just means there is not enough data to make any meaningful conclusions. 

### Graph
The program also outputs a graph titled `output.html` that can be viewed in any browser. This contains the same information described above, but in the form of a bar chart which may be easier for some people to understand or compare between states. 

## Running the CLI
Since there is limited features implemented currently, running `cargo run` from the directory in your terminal is all you need to do. 

## Working with the rtrend crate
The documentation for the rust rtrend crate can be found [here](https://crates.io/crates/rtrend). I also found an article using the python equivalent library to be very helpful in understanding more details about how the Google Trends API generally works. That article can be found [here](https://towardsdatascience.com/google-trends-api-for-python-a84bc25db88f).
