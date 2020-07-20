#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(dead_code)]
#![allow(clippy::module_name_repetitions)]

mod args;
mod chunks;
mod client;
mod http;
mod model;
mod setting;
mod template;

use args::Args;

pub use anyhow::Result;

fn unwrap_or_unknown<S: AsRef<str>>(s: &Option<S>) -> &str {
    s.as_ref().map(|s| s.as_ref()).unwrap_or("Unknown")
}

fn main() -> Result<()> {
    let argument = args::get();
    let client = client::Client::new()?;

    match argument {
        Args::Ip { ip } => {
            let data = client.ip(&ip)?;

            println!("     IP: {}", ip);

            let loc = &data.max_mind;

            println!("Country: {}", loc.country_code);
            println!("   City: {}", unwrap_or_unknown(&loc.city));

            let iana = &data.iana_assignment;
            println!();
            println!("   IANA: {}", iana.description);
            println!("AssigSt: {}", iana.assignment_status);
            println!("AssigAt: {}", unwrap_or_unknown(&iana.data_assigned));
            println!("  Whois: {}", iana.whois_server);

            let rir = &data.rir_allocation;
            println!();
            println!("    RIR: {} - {}", rir.rir_name, rir.country_code);
            println!(" Prefix: {}", rir.prefix);
            println!("AllocSt: {}", rir.allocation_status);
            if let Some(ref date) = rir.date_allocated {
                println!("AllocAt: {}", date);
            }

            let prefixes = &data.prefixes;
            if prefixes.is_empty() {
                println!();
                println!("No ASN owns {}", ip);
            } else {
                for prefix in prefixes {
                    println!();
                    let asn = &prefix.asn;
                    println!(
                        "    ASN: {} - {} - {} - {}",
                        asn.asn, asn.name, asn.description, asn.country_code,
                    );
                    println!(" Prefix: {}", prefix.prefix);
                    println!("   Name: {}", unwrap_or_unknown(&prefix.name));
                    println!("   Desc: {}", unwrap_or_unknown(&prefix.description));
                    println!("Country: {}", unwrap_or_unknown(&prefix.country_code));
                }
            }
        }
    }

    Ok(())
}
