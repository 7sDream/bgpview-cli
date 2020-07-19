use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpResponseData {
    pub ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response<Data> {
    pub status: String,
    pub status_message: String,
    pub data: Data,
}

pub type IpResponse = Response<IpResponseData>;
