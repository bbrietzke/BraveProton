use std::{sync::Arc, time::Duration};
use k8s_openapi::{api::networking::v1::Ingress, serde_json::{json, Value}};
use kube::{Api, runtime::controller::Action, ResourceExt, Resource, Client};

use crate::utils::{ KubernetesContext, OperatorActivities };


pub async fn reconciler(ingress: Arc<Ingress>, context: Arc<KubernetesContext>) -> Result<Action, kube::Error> {
    let name: String = ingress.name_any();
    let ns: String = ingress.namespace().expect("Must have a valid namespace");
    let update_seconds = context.update_seconds;

    return match determine_activity(ingress, update_seconds) {
        OperatorActivities::Create => {
            Ok(Action::await_change())
        },
        OperatorActivities::Delete => {
            Ok(Action::await_change())
        },
        OperatorActivities::Update(_) => {
            Ok(Action::await_change())
        },
    }
}



fn determine_activity(ingress: Arc<Ingress>, update_seconds: u64) -> OperatorActivities {
    return if ingress.meta().deletion_timestamp.is_some() {
        OperatorActivities::Delete
    } else if ingress.meta().finalizers.is_none() {
        OperatorActivities::Create 
    } else {
        OperatorActivities::Update(Duration::from_secs(update_seconds))
    }
}

async fn set_finalizer(client: Client, ingress_name: &str, ingress_namespace: &str, finalizer_value: &str) -> Result<Ingress, kube::Error> {
    let api: Api<Ingress> = Api::namespaced(client.clone(), ingress_namespace);
    let finalizer: Value = json!({
        "metadata": { "finalizers": finalizer_value }
    });

    let patch: kube::api::Patch<&Value> = kube::api::Patch::Merge(&finalizer);
    api.patch(ingress_name, &kube::api::PatchParams::default(), &patch).await
}