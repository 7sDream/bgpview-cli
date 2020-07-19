use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Asn {
    pub asn: u64,
    pub name: String,
    pub description: String,
    pub country_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Prefix {
    pub prefix: String,
    pub ip: String,
    pub cidr: u8,
    pub asn: Asn,
    pub name: String,
    pub description: String,
    pub country_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct IpResponseData {
    pub ip: String,
    pub prefixes: Vec<Prefix>,
}

#[derive(Serialize, Deserialize)]
pub struct Response<Data> {
    pub status: String,
    pub status_message: String,
    pub data: Data,
}

pub type IpResponse = Response<IpResponseData>;
