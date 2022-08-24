use futures_util::stream::StreamExt;
use kube::{error::ErrorResponse, ResourceExt};

pub async fn start_operations(api_token: &str, zone_id: &str) -> () {
    let kubectl: kube::client::Client = kube::client::Client::try_default()
        .await
        .expect("Please provide a valid kubernetes configuration.");

    let ingress_api: kube::Api<k8s_openapi::api::networking::v1::Ingress> = kube::Api::all(kubectl.clone());
    let context:std::sync::Arc<utils::Context> = std::sync::Arc::new(utils::Context{ 
        api_token: api_token.to_string(), 
        zone_id: zone_id.to_string(), 
        client: kubectl.clone() 
    });

    kube::runtime::Controller::new(ingress_api.clone(), kube::api::ListParams::default())
        .run(reconcile_ingress, on_error, context)
        .for_each(|result| async move {
            match result {
                Ok(ingress) => {
                    let i = ingress.0.clone();
                    log::info!("success with {:?}", i);
                }
                Err(error) => {
                    log::error!("error reconcilling : {:?}", error)
                }
            }
        })
        .await;

    return ();
}


async fn reconcile_ingress(ingress: std::sync::Arc<k8s_openapi::api::networking::v1::Ingress>, context:std::sync::Arc<utils::Context>) -> Result<kube::runtime::controller::Action, kube::Error> {
    let name: String = ingress.name_any();
    let namespace: String = match ingress.namespace() {
        Some(ns) => { ns },
        None => {
            return Err(kube::Error::Api(ErrorResponse{ 
                status:"failed".to_string(), 
                message:"invalid namespace".to_string(),  
                reason: "no namespace assigned".to_string(),
                code: 1
            }));
        }
    };

    let host_names: Vec<String> = match utils::get_host_name(&ingress) {
        None => {
            return Ok(kube::runtime::controller::Action::await_change());
        },
        Some(names) => {
            names
        }
    };

    let ip: String = match utils::get_ip_address(&ingress) {
        None => {
            return Ok(kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(2)));
        },
        Some(address) => { address }
    };

    return match utils::determine_activity(&ingress) {
        utils::OperatorActivities::Retry(value) => {
            log::info!("Retry");
            Ok(kube::runtime::controller::Action::requeue(value))
        },
        utils::OperatorActivities::Delete => {
            match activities::delete_entries(context.client.clone(), &name, &namespace, host_names, &context.api_token, &context.zone_id).await {
                Err(error) => {
                    return Err(error);
                },
                _ => {
                    Ok(kube::runtime::controller::Action::await_change())
                }
            }
        },
        utils::OperatorActivities::Create => {
            match activities::create_entries(context.client.clone(), &name, &namespace, host_names, ip, &context.api_token, &context.zone_id).await {
                Err(error) => {
                    return Err(error);
                },
                _ => {
                    Ok(kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(1800)))
                }
            }
        }
    };



    
}

fn on_error(error: &kube::Error, _context:std::sync::Arc<utils::Context>) -> kube::runtime::controller::Action {
    log::error!("{:?}", error);
    kube::runtime::controller::Action::await_change()
}

mod activities {
    pub async fn create_entries(client: kube::Client, name: &str, ns: &str, host_names: Vec<String>, ip: String, api_token: &str, zone_id: &str) -> Result<(), kube::Error> {
        match super::finalization::add_finalizer(client, name, ns).await {
            Err(error) => {
                return Err(error);
            },
            Ok(_ingress) => {
                for host in host_names {
                    log::info!("updating DNS for {:?} to be {:?}!", host, ip)
                }
            }
        };

        return Ok(());
    }

    pub async fn delete_entries(client: kube::Client, name: &str, ns: &str, host_names: Vec<String>, api_token: &str, zone_id: &str) -> Result<(), kube::Error> {
        match super::finalization::remove_finalizer(client, name, ns).await {
            Err(error) => {
                return Err(error);
            },
            Ok(_ingress) => {
                for host in host_names {
                    log::info!("removing DNS for {:?}", host)
                }
            }
        };

        return Ok(());
    }
}

mod utils {
    pub struct Context {
        pub api_token: String,
        pub zone_id: String,
        pub client: kube::client::Client
    }
    
    pub enum OperatorActivities {
        Create,
        Delete,
        Retry(std::time::Duration)
    }

    pub fn determine_activity(ingress: &k8s_openapi::api::networking::v1::Ingress) -> OperatorActivities {
        return if ingress.metadata.deletion_timestamp.is_some() {
            OperatorActivities::Delete
        } else if ingress.metadata.finalizers.as_ref().map_or(true, |f| f.is_empty()) {
            OperatorActivities::Create
        } else {
            OperatorActivities::Retry(std::time::Duration::from_secs(1800))
        }
    }

    pub fn get_host_name(ingress: &k8s_openapi::api::networking::v1::Ingress) -> Option<Vec<String>> {
        match &ingress.spec {
            Some(spec ) => {
                let mut v = Vec::new();

                for rule in spec.rules.as_ref().unwrap() {
                    log::info!("{:?}", &rule);
                    match rule.host.clone() {
                        Some(h) => { 
                            v.push(h)
                         }
                        _  => {

                        }
                    }
                }
                
                Some(v.clone())
            },
            _ => {
                None
            }
        }
    }

    pub fn get_ip_address(ingress: &k8s_openapi::api::networking::v1::Ingress) -> Option<String> {
        match &ingress.status {
            Some(status) => {
                let mut r:String = String::new();
                match &status.load_balancer.as_ref().unwrap().ingress {
                    None => {
                        None
                    },
                    Some(lbi) => {
                        for load_balancer in lbi {
                            r = load_balancer.ip.as_ref().unwrap().clone();
                        }

                        Some(r.clone())
                    }
                }
            },
            _ => {
                Some(String::new())
            }
        }
    }
}

mod finalization {
    use k8s_openapi::serde_json::{json, Value};

    pub async fn add_finalizer(client:kube::Client, name: &str, namespace: &str) -> Result<k8s_openapi::api::networking::v1::Ingress, kube::Error> {
        let api: kube::Api<k8s_openapi::api::networking::v1::Ingress> = kube::Api::namespaced(client, namespace);
        let finalizer: Value = json!({
            "metadata": {
                "finalizers": ["brave-proton.faultycloud.io/finalizer"]
            }
        });
        let patch: kube::api::Patch<&Value> = kube::api::Patch::Merge(&finalizer);

        api.patch(name, &kube::api::PatchParams::default(), &patch).await
    }

    pub async fn remove_finalizer(client:kube::Client, name: &str, namespace: &str) -> Result<k8s_openapi::api::networking::v1::Ingress, kube::Error> {
        let api: kube::Api<k8s_openapi::api::networking::v1::Ingress> = kube::Api::namespaced(client, namespace);
        let finalizer: Value = json!({
            "metadata": {
                "finalizers": []
            }
        });
        let patch: kube::api::Patch<&Value> = kube::api::Patch::Merge(&finalizer);

        api.patch(name, &kube::api::PatchParams::default(), &patch).await
    }
}