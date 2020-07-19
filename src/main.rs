#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(dead_code)]
#![allow(clippy::module_name_repetitions)]

mod chunks;
mod http;
mod model;
mod setting;
mod template;

use std::{io, time::Duration};

fn main() -> io::Result<()> {
    let client = http::Client::new(
        setting::api::SCHEMA,
        setting::api::DOMAIN,
        setting::api::PORT,
    )?;

    let path = template::fill(
        setting::api::ip::TEMPLATE,
        &maplit::hashmap! {
            "ip" => "1.1.1.1",
        },
    );

    let mut response = Vec::new();
    let body = client.request(
        setting::api::ip::METHOD,
        path,
        http::NONE,
        http::NONE,
        None,
        &mut response,
        Duration::from_millis(2000),
    )?;

    let ip_resp = serde_json::from_reader::<_, model::IpResponse>(body).unwrap();

    println!("Ip: {}", ip_resp.data.ip);

    Ok(())
}
