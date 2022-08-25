use std::sync::Arc;
use k8s_openapi::api::networking::v1::Ingress;
use kube::{runtime::controller::Action, ResourceExt};

use crate::utils::KubernetesContext;


pub fn reconciler(ingress: Arc<Ingress>, context: Arc<KubernetesContext>) -> Result<Action, kube::Error> {
    let name: String = ingress.name_any();
    let ns: String = ingress.namespace().expect("Must have a valid namespace");

    Ok(Action::await_change())
}