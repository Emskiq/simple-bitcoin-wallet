extern crate dotenv;

use std::env;

fn main() {
    println!("Hello, world!");

    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();

    println!("{}", descriptor);
}

