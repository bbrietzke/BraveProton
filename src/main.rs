mod operator; 
mod utils;
mod cloudflare;

use crate::operator::start_operator;

#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = utils::init();

    let zone_id = matches.value_of("zone-id").unwrap();
    let api_token = matches.value_of("api-token").unwrap();


    println!("{:?}", zone_id);
    println!("{:?}", api_token);

    match start_operator(api_token.to_string(), zone_id.to_string()).await {
        Err(e) => panic!("{:?}", e),
        _ => print!("We're okay!")
    }
}
