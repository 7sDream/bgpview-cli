use percent_encoding::{AsciiSet, CONTROLS};

pub const QUERY_ASCII_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
pub const PATH_ASCII_SET: &AsciiSet = &QUERY_ASCII_SET.add(b'?').add(b'`').add(b'{').add(b'}');

pub mod api {
    use {once_cell::sync::Lazy, std::collections::HashMap};

    pub const SCHEMA: &str = "https";
    pub const DOMAIN: &str = "api.bgpview.io";
    pub const PORT: u16 = 443;
    pub const HEADERS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        maplit::hashmap! {
            "Accept-encoding" => "identity",
            "Accept" => "application/json",
            "Content-Length" => "0",
        }
    });
    pub mod ip {
        pub const METHOD: &str = "GET";
        pub const TEMPLATE: &str = "/ip/{ip}";
    }
}
