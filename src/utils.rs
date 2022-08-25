use std::time::Duration;

use kube::Client;


pub struct KubernetesContext {
    pub client: Client,
    pub update_seconds: u64,
}

pub enum OperatorActivities {
    Create,
    Delete,
    Update(Duration),
}