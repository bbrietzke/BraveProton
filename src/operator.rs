use std::sync::Arc;
use futures_util::stream::StreamExt;
use k8s_openapi::api::networking::v1::Ingress;
use kube::{Client, Api, api::ListParams, runtime::{Controller, controller::Action}};

use crate::utils::KubernetesContext;
use crate::reconciller::reconciler;
use crate::error::{Result};



pub async fn start_operations() -> Result<()> {
    let kubectl: Client = Client::try_default().await?;

    let ingress_api: Api<Ingress> = Api::all(kubectl.clone());
    let context: Arc<KubernetesContext>  = Arc::new(KubernetesContext {
        client: kubectl.clone(),
        update_seconds: 1800,
    }); 

    Controller::new(ingress_api.clone(), ListParams::default())
        .run(reconciler, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok(i) => {
                    let ingress = i.0.clone();
                    match ingress.namespace {
                        Some(ns) => { log::warn!("Reconciled ingress {:?}.{:?}", ns, ingress.name); },
                        _ => { log::warn!("Reconciled {:?}", ingress.name) }
                    }
                },
                Err(e) => {
                    match e {
                        kube::runtime::controller::Error::ObjectNotFound(obj) => {
                            log::error!("Object Not Found: {:?}", obj);
                        },
                        kube::runtime::controller::Error::ReconcilerFailed(e, obj) => {
                            log::error!("Reconciller Error: [{:?}] {:?}", e, obj);
                        },
                        kube::runtime::controller::Error::QueueError(e) => {
                            match e {
                                kube::runtime::watcher::Error::InitialListFailed(e) => {
                                    log::error!("Initial List Failed {:?}", e);
                                },
                                kube::runtime::watcher::Error::WatchStartFailed(e) => {
                                    log::error!("Watch Start Failed {:?}", e);
                                },
                                kube::runtime::watcher::Error::WatchError(e) => {
                                    log::error!("Watch Error {:?}", e);
                                },
                                kube::runtime::watcher::Error::WatchFailed(e) => {
                                    log::error!("Watch Failed: {:?}", e);
                                },
                                kube::runtime::watcher::Error::TooManyObjects => {
                                    log::error!("Too Many Objects");
                                },
                            }
                        },
                    }
                }
            }
        }).await;
        
    Ok(())
}

fn error_policy(error: &kube::Error, _context: Arc<KubernetesContext>) -> Action {
    log::error!("error_policy => {:?}", error);
    Action::await_change()
}