use {
    crate::{
        http::{self, Client as HTTPClient},
        model::{IpResponse, IpResponseData},
        setting, template, Result,
    },
    std::{
        fmt::{self, Formatter},
        net::IpAddr,
        time::Duration,
    },
};

#[derive(Debug, Eq, PartialEq)]
pub enum ResponseError {
    JsonDeserializeFailed(String),
    StatusNotOk,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for ResponseError {}

pub struct Client {
    http: HTTPClient<'static>,
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            http: HTTPClient::new(
                setting::api::SCHEMA,
                setting::api::DOMAIN,
                setting::api::PORT,
            )?,
        })
    }

    pub fn ip(&self, ip: &IpAddr) -> Result<IpResponseData> {
        let path = template::fill(
            setting::api::ip::TEMPLATE,
            &maplit::hashmap! {
                "ip" => ip.to_string(),
            },
        );

        let mut buffer = Vec::new();
        let resp = self.http.request(
            setting::api::ip::METHOD,
            path,
            http::NONE,
            http::NONE,
            None,
            &mut buffer,
            Duration::from_millis(2000),
        )?;

        if resp.code != 200 {
            Err(http::HttpClientError::StatusCodeNotOk)?;
        }

        let ip_resp = serde_json::from_reader::<_, IpResponse>(resp.body)
            .map_err(|err| ResponseError::JsonDeserializeFailed(err.to_string()))?;

        if ip_resp.status != "ok" {
            Err(ResponseError::StatusNotOk)?;
        }

        Ok(ip_resp.data)
    }
}
