# Programming Trends
![worflow status](https://github.com/brycewhit13/rust-miniprojects/actions/workflows/rust.yml/badge.svg)

## Motivation
The goal of this project is to analyze google search trends to determine what is currently hot in the programming world and what may be dying off or up and coming. This project uses [Google Trends](https://trends.google.com/trends/?geo=US) using the [rtrend](https://crates.io/crates/rtrend) package in rust, which is an unnoficial API for interacting with Google Trend.

## Understanding the Output

## Running the CLI
Since there is limited features implemented currently, running `cargo run` from the directory in your terminal is all you need to do. 

## Working with the rtrend crate
The documentation for the rust rtrend crate can be found [here](https://crates.io/crates/rtrend). I also found an article using the python equivalent library to be very helpful in understanding more details about how the Google Trends API generally works. That article can be found [here](https://towardsdatascience.com/google-trends-api-for-python-a84bc25db88f).
