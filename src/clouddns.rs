
use crate::error::{Result, AppError};
use cloudflare::framework::OrderDirection;
use cloudflare::framework::{async_api::Client, auth::Credentials, HttpApiClientConfig, Environment};
use cloudflare::endpoints::dns::{ListDnsRecords, ListDnsRecordsParams};

fn create_cloudflare_client(api_token: &str) -> Result<Client> {
    let creds : Credentials = Credentials::UserAuthToken { token: api_token.to_string() };

    Ok(Client::new(creds, HttpApiClientConfig::default(), Environment::Production)?)
}

pub async fn get_cloud_record(api_token: &str, zone_id: &str, name: &str) -> Result<Option<String>> {
    let client = create_cloudflare_client(&api_token)?;
    let results = match client.request_handle(&ListDnsRecords {
            zone_identifier: zone_id,
            params: ListDnsRecordsParams {
                direction: Some(OrderDirection::Descending),
                name: Some(name.to_string()),
                ..Default::default()
            }}).await {
        Ok(it) => it,
        Err(err) => return Err(AppError::from(err)),
    }.result;

        if results.first().is_some() {
            Ok(Some(results[0].id.clone()))
        } else {
            Ok(None)
        }
}