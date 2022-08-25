use kube::error::ErrorResponse;
use kube::Error;

pub type Result<T> = std::result::Result<T, AppError>;

pub enum AppError {
    DnsFailedToCreate,
    DnsFailedToDelete,
    DnsFailedToUpdate,
    Kubernetes(kube::Error),
    ApiError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::DnsFailedToCreate => { "failed to create the DNS entry" },
            Self::DnsFailedToDelete => { "failed to delete the DNS entry" },
            Self::DnsFailedToUpdate => { "failed to update the DNS entry" },
            Self::Kubernetes(_) => { "Kubernetes freaked out" },
            Self::ApiError(_) => { "Something is wrong with the manifest" },
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
            Self::ApiError(_) => { "Something is wrong with the manifest" },
        };

        write!(f, "[ file: {}] [line: {}] {}", file!(), line!(), msg)
    }
}

impl From<kube::Error> for AppError {
    fn from(error: kube::Error) -> Self {
        AppError::Kubernetes(error)
    }
}

impl Into<kube::Error> for AppError {
    fn into(self) -> kube::Error {
        match self {
            AppError::DnsFailedToCreate => {
                return Error::Api(ErrorResponse {
                    status: String::from("failed"),
                    message: String::from("failed to create the DNS entry"),
                    reason:  String::from("failed to create the DNS entry"),
                    code: 20u16
                });
            },
            AppError::DnsFailedToDelete => {
                return Error::Api(ErrorResponse {
                    status: String::from("failed"),
                    message: String::from("failed to delete the DNS entry"),
                    reason:  String::from("failed to delete the DNS entry"),
                    code: 20u16
                });
            },
            AppError::DnsFailedToUpdate => {
                return Error::Api(ErrorResponse {
                    status: String::from("failed"),
                    message: String::from("failed to update the DNS entry"),
                    reason:  String::from("failed to update the DNS entry"),
                    code: 20u16
                });
            },
            AppError::Kubernetes(e) => { return e; },
            AppError::ApiError(_) => {
                return Error::Api(ErrorResponse {
                    status: String::from("failed"),
                    message: String::from("something in wrong with the ingress manifest"),
                    reason:  String::from("something in wrong with the ingress manifest"),
                    code: 10u16
                });
            },
        }
    }
}