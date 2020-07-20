use {
    serde::de,
    serde::Deserialize,
    std::fmt::{self, Formatter},
};

struct DeserializeCIDR;

impl<'de> serde::de::Visitor<'de> for DeserializeCIDR {
    type Value = u16;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("an cidr number or string which has range [0, 128]")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v > 128 {
            Err(E::invalid_value(de::Unexpected::Unsigned(v), &self))
        } else {
            Ok(v as u16)
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "" {
            Ok(0)
        } else {
            v.parse()
                .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}

fn deserialize_cidr<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeCIDR)
}

#[derive(Debug, Deserialize)]
pub struct Asn {
    pub asn: u64,
    pub name: String,
    pub description: String,
    pub country_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Prefix {
    pub prefix: String,
    pub ip: String,
    #[serde(deserialize_with = "deserialize_cidr")]
    pub cidr: u16,
    pub asn: Asn,
    pub name: Option<String>,
    pub description: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RIRAllocation {
    pub rir_name: String,
    pub country_code: String,
    pub ip: String,
    #[serde(deserialize_with = "deserialize_cidr")]
    pub cidr: u16,
    pub prefix: String,
    pub allocation_status: String,      // TODO: maybe a enum?
    pub date_allocated: Option<String>, // TODO: maybe use chrono::Date someday?
}

#[derive(Debug, Deserialize)]
pub struct IANAAssignment {
    pub assignment_status: String,
    pub description: String,
    pub whois_server: String,
    pub data_assigned: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MaxMind {
    pub country_code: String,
    pub city: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IpResponseData {
    pub ip: String,
    pub prefixes: Vec<Prefix>,
    pub rir_allocation: RIRAllocation,
    pub iana_assignment: IANAAssignment,
    #[serde(rename = "maxmind")]
    pub max_mind: MaxMind,
}

#[derive(Debug, Deserialize)]
pub struct Response<Data> {
    pub status: String,
    pub status_message: String,
    pub data: Data,
}

pub type IpResponse = Response<IpResponseData>;
