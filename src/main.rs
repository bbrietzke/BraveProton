mod operator;
mod utils;
mod reconciller;
mod error;
mod clouddns;


use crate::operator::start_operations;

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    env_logger::init();

    match start_operations().await {
        Err(e) => {
            Err(e)
        },
        _ => { Ok(()) }
    }
}
