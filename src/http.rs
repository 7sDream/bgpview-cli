use {
    crate::{chunks::BodyChunks, setting},
    percent_encoding::utf8_percent_encode as urlencode,
    rustls::{ClientConfig, ClientSession},
    std::{
        collections::HashMap,
        io::{self, Read, Write},
        net::{SocketAddr, TcpStream, ToSocketAddrs},
        sync::Arc,
        time::Duration,
    },
};

pub static NONE: &Option<HashMap<&'static str, &'static str>> = &None;

#[must_use]
fn write_head<W: Write, K: AsRef<str>, V: AsRef<str>>(tls: &mut W, k: K, v: V) -> io::Result<()> {
    tls.write_fmt(format_args!("{}: {}\r\n", k.as_ref(), v.as_ref()))
}

#[must_use]
fn write_heads<W: Write, K: AsRef<str>, V: AsRef<str>>(
    tls: &mut W,
    headers: &HashMap<K, V>,
) -> io::Result<()> {
    for (k, v) in headers {
        write_head(tls, k, v)?;
    }
    Ok(())
}

pub struct Client<'s> {
    config: Arc<ClientConfig>,
    schema: &'s str,
    domain: &'s str,
    port: u16,
    addr: SocketAddr,
}

impl<'s> Client<'s> {
    pub fn new(schema: &'s str, domain: &'s str, port: u16) -> io::Result<Self> {
        let mut config = rustls::ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        let config = Arc::new(config);

        let addr = format!("{}:{}", domain, port)
            .to_socket_addrs()?
            .next()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Send request failed: can't get host ip",
            ))?;

        Ok(Self {
            config,
            schema,
            domain,
            port,
            addr,
        })
    }

    fn parse_response<'buf, R: Read>(
        tls: &mut R,
        buffer: &'buf mut Vec<u8>,
    ) -> std::io::Result<BodyChunks<'buf>> {
        tls.read_to_end(buffer)?;

        let mut response_headers = [httparse::Header {
            name: "",
            value: &buffer,
        }; 32];

        let mut response = httparse::Response::new(&mut response_headers);

        let body = match response.parse(&buffer) {
            Ok(status) if status.is_complete() => {
                let size = status.unwrap();
                &buffer[size..]
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Received bytes is not valid HTTP response",
                ))
            }
        };

        let code = response.code.unwrap();
        if code != 200 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Received  HTTP response code: {}", code),
            ));
        }

        let content_length = response_headers
            .iter()
            .find(|h| h.name.to_ascii_lowercase() == "content-length")
            .iter()
            .filter_map(|h| std::str::from_utf8(h.value).ok())
            .next()
            .iter()
            .filter_map(|value| value.parse::<usize>().ok())
            .next();

        let mut body_chunks = Vec::new();

        if let Some(content_length) = content_length {
            body_chunks.push(&body[..content_length]);
        } else {
            let transfer_encoding = response_headers
                .iter()
                .find(|h| h.name.to_ascii_lowercase() == "transfer-encoding")
                .iter()
                .filter_map(|h| std::str::from_utf8(h.value).ok())
                .next();

            let mut start = 0;
            match transfer_encoding {
                Some("chunked") => loop {
                    match httparse::parse_chunk_size(&body[start..]) {
                        // end chunk has 0 size
                        Ok(httparse::Status::Complete((_, 0))) => {
                            break;
                        }
                        // normal chunk has a pos and size
                        Ok(httparse::Status::Complete((pos, size))) => {
                            body_chunks.push(&body[pos..(pos + size as usize)]);
                            start = pos + size as usize + 2;
                        }
                        // other condition is error
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "HTTP response body invalid",
                            ));
                        }
                    }
                },
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unknown HTTP response transfer-encoding",
                    ));
                }
            }
        };

        Ok(BodyChunks::new(body_chunks))
    }

    pub fn request<'buf, M, P, QK, QV, HK, HV>(
        &self,
        method: M,
        path: P,
        query: &Option<HashMap<QK, QV>>,
        headers: &Option<HashMap<HK, HV>>,
        body: Option<&[u8]>,
        buffer: &'buf mut Vec<u8>,
        timeout: Duration,
    ) -> std::io::Result<BodyChunks<'buf>>
    where
        M: AsRef<str>,
        P: AsRef<str>,
        QK: AsRef<str>,
        QV: AsRef<str>,
        HK: AsRef<str>,
        HV: AsRef<str>,
    {
        let domain = webpki::DNSNameRef::try_from_ascii_str(self.domain).unwrap();
        let mut session = ClientSession::new(&self.config, domain);

        let mut socket = TcpStream::connect_timeout(&self.addr, timeout)?;
        let mut tls = rustls::Stream::new(&mut session, &mut socket);

        tls.write(method.as_ref().as_bytes())?;
        tls.write(b" ")?;
        tls.write(path.as_ref().as_bytes())?;

        if let Some(query) = query {
            if !query.is_empty() {
                tls.write(b"?")?;
                for (i, (k, v)) in query.iter().enumerate() {
                    for s in urlencode(k.as_ref(), setting::QUERY_ASCII_SET) {
                        tls.write(s.as_bytes())?;
                    }
                    tls.write(b"=")?;
                    for s in urlencode(v.as_ref(), setting::QUERY_ASCII_SET) {
                        tls.write(s.as_bytes())?;
                    }
                    if i != query.len() - 1 {
                        tls.write(b"&")?;
                    }
                }
            }
        }

        tls.write(b" HTTP/1.1\r\n")?;

        write_head(&mut tls, "Host", self.domain)?;
        write_head(&mut tls, "Connection", "close")?;

        if let Some(body) = body {
            write_head(&mut tls, "Content-Length", body.len().to_string())?;
        }

        if let Some(headers) = headers {
            write_heads(&mut tls, headers)?;
        }

        tls.write(b"\r\n")?;

        if let Some(body) = body {
            tls.write(body)?;
        }

        Self::parse_response(&mut tls, buffer)
    }
}
