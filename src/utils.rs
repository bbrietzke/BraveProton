
pub mod args {
    use clap::{Arg, App};

    pub fn init() -> clap::ArgMatches {
        return App::new("BraveProton")
            .version("0.1.0")
            .about("Operator to configure DNS for kubernetes ingress policies.")
            .author("Brian Brietzke")
            .arg(Arg::with_name("api-token")
                .long("api-token")
                .short('t')
                .required(true)
                .takes_value(true)
                .env("API_TOKEN")
                .help("The API token for Cloudflare."))
            .arg(Arg::with_name("zone-id")
                .long("zone-id")
                .short('z')
                .required(true)
                .takes_value(true)
                .env("ZONE_ID")
                .help("The zone to modify as provided by Cloudflare"))
            .arg(Arg::with_name("ip")
                .long("ip")
                .required(true)
                .takes_value(true)
                .env("IP")
                .help("The IP address to map to"))
            .get_matches();
    }
}