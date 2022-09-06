use std::{sync::Arc, time::Duration};
use k8s_openapi::{api::networking::v1::Ingress, serde_json::{json, Value}};
use kube::{Api, runtime::controller::Action, ResourceExt, Resource, Client};

use crate::utils::{ KubernetesContext, OperatorActivities };
use crate::error::{Result, AppError};


pub async fn reconciler(ingress: Arc<Ingress>, context: Arc<KubernetesContext>) -> std::result::Result<Action, kube::Error> {
    let name: String = ingress.name_any();
    let ns: String = ingress.namespace().expect("Must have a valid namespace");
    let update_seconds: u64 = context.update_seconds;

    return match determine_activity(ingress, update_seconds) {
        OperatorActivities::Create => {
            match update_ingress(context.client.clone(), &name, &ns, update_seconds).await {
                Ok(action) => { return Ok(action); },
                Err(e) => { return Err(e.into()); },
            }
        },
        OperatorActivities::Delete => {
            match delete_ingress(context.client.clone(), &name, &ns).await {
                Ok(action) => { return Ok(action); },
                Err(e) => { return Err(e.into()); },
            }
        },
        OperatorActivities::Update(duration) => {
            Ok(Action::requeue(duration))
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

async fn update_ingress(client: Client, ingress_name: &str, ingress_namespace: &str, update_seconds: u64) -> Result<Action> {
    match set_finalizer(client.clone(), ingress_name, ingress_namespace ).await {
        Err(e) => { return Err(AppError::from(e)) },
        Ok(value) => {
            match &value.status {
                Some(status) => {
                    match &status.load_balancer.as_ref().unwrap().ingress {
                        None => {  },
                        Some(balancers) => {
                            for b in balancers {
                                match &b.ip  {
                                    Some(ip_address) => { ip_address.clone() },
                                    None => {
                                        match &b.hostname {
                                            Some(host) => { host.to_string() },
                                            None  => { 
                                                return Err(AppError::ApiError(String::from("No hostname or IP address could be returned.")));
                                            }
                                        }
                                    },
                                };
                            }
                        }
                    }
                },
                None => {
                    return Err(AppError::ApiError(String::from("no status clause found for ingress")));
                }
            };
            let mut hosts: Vec<String> = Vec::<String>::new();
            match &value.spec {
                Some(spec) => {
                    for rule in spec.rules.as_ref().unwrap() {
                        match &rule.host {
                            Some(host) => { hosts.push(host.clone()); }
                            None => { }
                        };
                    }

                    // Add dns updates here
                },
                None => {
                    return Err(AppError::ApiError(String::from("no spec clause found for ingress")));
                }
            }

        },
    } 
    
    Ok(Action::requeue(Duration::from_secs(update_seconds)))
}

async fn delete_ingress(client: Client, ingress_name: &str, ingress_namespace: &str) -> Result<Action> {
    match delete_finalizer(client.clone(), ingress_name, ingress_namespace).await {
        Err(e) => { return Err(AppError::from(e)) },
        Ok(_value) => {
            log::info!("removing DNS entries.");
            // delete DNS entries here
        },
    } 
    Ok(Action::await_change())
}

async fn set_finalizer(client: Client, ingress_name: &str, ingress_namespace: &str) -> std::result::Result<Ingress, kube::Error> {
    log::info!("adding finalizer.");
    let api: Api<Ingress> = Api::namespaced(client.clone(), ingress_namespace);
    let finalizer: Value = json!({
        "metadata": { "finalizers": ["brave-proton.faultycloud.io/finalizer"] }
    });

    let patch: kube::api::Patch<&Value> = kube::api::Patch::Merge(&finalizer);
    api.patch(ingress_name, &kube::api::PatchParams::default(), &patch).await
}

async fn delete_finalizer(client: Client, ingress_name: &str, ingress_namespace: &str) -> std::result::Result<Ingress, kube::Error> {
    log::info!("removing finalizer.");
    let api: Api<Ingress> = Api::namespaced(client.clone(), ingress_namespace);
    let finalizer: Value = json!({
        "metadata": { "finalizers": [] }
    });

    let patch: kube::api::Patch<&Value> = kube::api::Patch::Merge(&finalizer);
    api.patch(ingress_name, &kube::api::PatchParams::default(), &patch).await
}