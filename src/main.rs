mod operator;
mod utils;
mod reconciller;

use crate::operator::start_operations;

#[tokio::main]
async fn main() -> crate::Result<()> {
    env_logger::init();

    match start_operations().await {
        Err(e) => {
            Err(e)
        },
        _ => { Ok(())}
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

pub enum AppError {
    DnsFailedToCreate,
    DnsFailedToDelete,
    DnsFailedToUpdate,
    Kubernetes(kube::Error)
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = String::new();
        let msg = match self {
            Self::DnsFailedToCreate => { "failed to create the DNS entry" },
            Self::DnsFailedToDelete => { "failed to delete the DNS entry" },
            Self::DnsFailedToUpdate => { "failed to update the DNS entry" },
            Self::Kubernetes(_) => { "Kubernetes freaked out" },
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::DnsFailedToCreate => { "failed to create the DNS entry" },
            Self::DnsFailedToDelete => { "failed to delete the DNS entry" },
            Self::DnsFailedToUpdate => { "failed to update the DNS entry" },
            Self::Kubernetes(_) => { "Kubernetes freaked out" },
        };

        write!(f, "[ file: {}] [line: {}] {}", file!(), line!(), msg)
    }
}

impl From<kube::Error> for AppError {
    fn from(error: kube::Error) -> Self {
        AppError::Kubernetes(error)
    }
}