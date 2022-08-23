use operator::start_operations;

mod utils;
mod operator;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = utils::args::init();

    let ip_address = matches.value_of("ip").expect("must provide an ip address");
    let zone_id = matches.value_of("zone-id").expect("must provide a zone id");
    let api_token = matches.value_of("api-token").expect("must provide api token");

    start_operations(api_token, zone_id, ip_address).await;
}
