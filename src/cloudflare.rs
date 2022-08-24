use cloudflare::{framework::{OrderDirection}, endpoints::dns::DnsRecord};
use cloudflare::framework::apiclient::ApiClient;




fn create_client(api_token: &str) -> Result<cloudflare::framework::HttpApiClient, anyhow::Error> {
    let creds: cloudflare::framework::auth::Credentials = cloudflare::framework::auth::Credentials::UserAuthToken{ token: api_token.to_string() };
    Ok(cloudflare::framework::HttpApiClient::new(creds, cloudflare::framework::HttpApiClientConfig::default(), cloudflare::framework::Environment::Production)?)
}


pub async fn list_records(api_token: &str, zone_id: &str) -> Result<Vec<DnsRecord>, anyhow::Error> {
    let client = create_client(api_token)?;
    let results = client.request(&cloudflare::endpoints::dns::ListDnsRecords {
        zone_identifier: &zone_id.to_string(), params: cloudflare::endpoints::dns::ListDnsRecordsParams{ direction: Some(OrderDirection::Ascending), ..Default::default()}})?.result;

    Ok(results)
}

pub async fn has_record(api_token: &str, zone_id: &str, name: &str) -> Result<bool, anyhow::Error> {
    let client = create_client(api_token)?;
    let results = client.request(&cloudflare::endpoints::dns::ListDnsRecords {
        zone_identifier: &zone_id.to_string(), params: cloudflare::endpoints::dns::ListDnsRecordsParams{ direction: Some(OrderDirection::Ascending), name: Some(name.to_string()), ..Default::default()}})?.result;

    Ok(results.len() == 1)
}

pub async fn update_record(api_token: &str, zone_id: &str, name: &str, ip: &str)  -> Result<(), anyhow::Error> {
    let client = create_client(api_token)?;

    Ok(())
}