use clap::App;
use operator::start_operations;

mod utils;
mod operator;
mod cloudflared;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = utils::args::init();

    let zone_id = matches.value_of("zone-id").expect("must provide a zone id");
    let api_token = matches.value_of("api-token").expect("must provide api token");

    start_operations(api_token, zone_id).await;
}


enum AppError {
    Kubernetes(kube::Error),
    DnsFailedToUpdate,
    DnsFailedToCreate,
    DnsFailedToDelete,
    TimeOut
}

type Result<T> = std::result::Result<T, AppError>;

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Something bad happened, don't panic")
    }
}

impl From<kube::Error> for AppError {
    fn from(error: kube::Error) -> Self {
        AppError::Kubernetes(error)
    }
}