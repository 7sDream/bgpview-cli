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

fn main() -> Result<()> {
    let argument = args::get();
    let client = client::Client::new()?;

    match argument {
        Args::Ip { ip } => {
            let data = client.ip(&ip)?;
            let prefixes = &data.prefixes;

            if prefixes.is_empty() {
                println!("No ASN owns {}", ip);
            } else {
                println!("{}", ip);
                for prefix in prefixes {
                    let asn = &prefix.asn;
                    println!();
                    println!(
                        "ASN{}: {} - {} - {}",
                        asn.asn, asn.name, asn.description, asn.country_code,
                    );
                    println!(" Prefix: {}", prefix.prefix);
                    println!("   Name: {}", prefix.name);
                    println!("   Desc: {}", prefix.description);
                    println!("Country: {}", prefix.country_code);
                }
            }
        }
    }

    Ok(())
}
