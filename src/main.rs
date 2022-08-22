
use cloudflare::endpoints::{ dns };
use cloudflare::framework::{
    apiclient::ApiClient,
    auth::Credentials,
    Environment, HttpApiClient, HttpApiClientConfig, OrderDirection,
};


fn main() {
    println!("Hello, world!");

    let zone_id = "";
    let name = "red.faultycloud.xyz";
    let token = "";

    let creds : Credentials = Credentials::UserAuthToken { token: token.to_string() };

    if let Ok(client) = HttpApiClient::new(creds, HttpApiClientConfig::default(), Environment::Production,){
        let r =  client.request(&dns::ListDnsRecords {
            zone_identifier: zone_id,
            params: dns::ListDnsRecordsParams {
                direction: Some(OrderDirection::Ascending),
                name: Some(name.to_string()),
                ..Default::default()
            },
        });

        if let Ok(results) = r {

            if results.result.len() == 1 {
                println!("We have a winner!");
                let content = &results.result[0];

                let u = client.request(&dns::UpdateDnsRecord {
                    zone_identifier: &content.zone_id,
                    identifier: &content.id,
                    params: dns::UpdateDnsRecordParams {
                        name: &content.name,
                        proxied: Some(content.proxied.clone()),
                        ttl: Some(content.ttl.clone()),
                        content: dns::DnsContent::A { content: std::net::Ipv4Addr::new(10, 0, 0, 30) },
                    }
                });
                    
                println!("{:?}", u);





            } else if results.result.len() > 1 {
                println!("To many results");
            }

            let records = results.result;

            for record in &records {
                println!("{:?}", record)
            }
        } else {
            println!("Something bad happened, maybe a bad zone id or api-token?");
            println!("{:?}", r);
        }


    }


}



/*

curl -X PUT "https://api.cloudflare.com/client/v4/zones/201ad99a8d8e1d0648fab41d1f04559c/dns_records/acf31e678877d09dbf2d812c1e1638c4" \
     -H "X-Auth-Email: user@example.com" \
     -H "X-Auth-Key: c2547eb745079dac9320b638f5e225cf483cc5cfdda41" \
     -H "Content-Type: application/json" \
     --data '{"type":"A","name":"example.com","content":"127.0.0.1","ttl":3600,"proxied":false}'


*/