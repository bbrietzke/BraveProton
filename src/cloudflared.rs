use std::{str::FromStr, any};

use cloudflare::{framework::{OrderDirection}};

fn create_client(api_token: &str) -> Result<cloudflare::framework::async_api::Client, anyhow::Error> {
    let creds: cloudflare::framework::auth::Credentials = cloudflare::framework::auth::Credentials::UserAuthToken{ token: api_token.to_string() };
    Ok(cloudflare::framework::async_api::Client::new(creds, cloudflare::framework::HttpApiClientConfig::default(), cloudflare::framework::Environment::Production)?)
}

pub async fn get_record(api_token: String, zone_id: String, name: String) -> Result<Option<String>, anyhow::Error> {
    let client = create_client(&api_token)?;
    let results = client.request_handle(&cloudflare::endpoints::dns::ListDnsRecords {
        zone_identifier: &zone_id.to_string(), params: cloudflare::endpoints::dns::ListDnsRecordsParams{ direction: Some(OrderDirection::Ascending), name: Some(name.to_string()), ..Default::default()}}).await?.result;
    
    if results.first().is_some() {
        Ok(Some(results[0].id.clone()))
    } else {
        Ok(None)
    }
}

pub async fn create_record(api_token: &str, zone_id: &str, names: Vec<String>, ip: &str)  -> Result<(), anyhow::Error> {
    let client = create_client(api_token)?;

    for name in names {
        let results = client.request_handle(&cloudflare::endpoints::dns::CreateDnsRecord {
            zone_identifier: zone_id,
            params: cloudflare::endpoints::dns::CreateDnsRecordParams{
                ttl: None,
                priority: Some(0),
                proxied: Some(false),
                name: &name,
                content: cloudflare::endpoints::dns::DnsContent::A { content: std::net::Ipv4Addr::from_str(&ip)? },
            }
        }).await;

        match results {
            Err(e) => {
                log::error!("{:?}", e);
                return Err(anyhow::Error::new(e));
            },
            Ok(value) => {
                log::debug!("{:?}", value);
            }
        }
    }

    Ok(())
}

pub async fn delete_record(api_token: &str, zone_id: &str, names: Vec<String>) -> Result<(), anyhow::Error> {
    let client = create_client(api_token)?;

    for name in names {
        match get_record(api_token.to_string(), zone_id.to_string(), name.to_string()).await {
            Ok(x) => {
                match x {
                    None => { },
                    Some(id) => {
                        match client.request_handle(&cloudflare::endpoints::dns::DeleteDnsRecord{
                            zone_identifier: zone_id,
                            identifier: &id
                        }).await {
                            Ok(value) => {
                                log::debug!("{:?}", value);
                            },
                            Err(e)  => {
                                log::error!("{:?}", e);
                                return Err(anyhow::Error::new(e));
                            }
                        }
                    }
                }
            },
            Err(e) => {
                log::error!("{:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

pub async fn update_record(api_token: &str, zone_id: &str, names: Vec<String>, ip: &str)  -> Result<(), anyhow::Error> {
    let client = create_client(api_token)?;

    for name in names {
        match get_record(api_token.to_string(), zone_id.to_string(), name.to_string()).await {
            Ok(x) => {
                match x {
                    None => { },
                    Some(id) => {
                        match client.request_handle(&cloudflare::endpoints::dns::UpdateDnsRecord {
                            zone_identifier: zone_id,
                            identifier: &id,
                            params: cloudflare::endpoints::dns::UpdateDnsRecordParams {
                                content: cloudflare::endpoints::dns::DnsContent::A { content: std::net::Ipv4Addr::from_str(&ip)? },
                                ttl: Some(1),
                                proxied: Some(false),
                                name: &name
                            }
                            
                        }).await {
                            Ok(value) => {
                                log::debug!("{:?}", value);
                            },
                            Err(e) => {
                                log::error!("{:?}", e);
                                return Err(anyhow::Error::new(e));
                            }
                        }
                    }
                }

            },
            Err(e) => {
                log::error!("{:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

