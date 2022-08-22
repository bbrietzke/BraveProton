use log::info;

use crate::utils::args;

mod utils;

fn main() {
    env_logger::init();
    info!("Hello World!");
    args::test_function();
}
