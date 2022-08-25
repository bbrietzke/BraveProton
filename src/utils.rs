use std::time::Duration;

use kube::Client;


pub struct KubernetesContext {
    pub client: Client,
}

pub enum OperatorActivities {
    Create,
    Delete,
    Update(Duration),
}