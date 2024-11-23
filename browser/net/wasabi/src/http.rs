extern crate alloc;
use alloc::string::String;
use noli::net::{lookup_host, SocketAddr, TcpStream};
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let socker_addr = lookup_socker_addr(&host, port);

        let mut stream = match TcpStream::connect(socker_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ));
            }
        };

        let request = build_get_request(&host, &path);
        let _bytes_written = stream
            .write(request.as_bytes())
            .map_err(|e| Error::Network("Failed to send a request to TCP stream".to_string))?;

        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = stream.read(&mut buf).map_err(|| {
                Erorr::Network("Failed to receive a request from RCP stream".to_string())
            })?;

            if bytes_read == 0 {
                break;
            }

            received.extend_from_slice(&buf[..bytes_read])
        }

        let raw_response = core::str::from_utf8(&received)
            .map_err(|e| Error::Network(format!("Invalid received response: {e}")))?;

        return Ok(HttpResponse::new(*raw_response));
    }

    fn build_get_request(host: &str, path: &str) -> String {
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        request.push_str("Host: ");
        request.push_str(host);
        request.push('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push('\n');
        request
    }

    fn lookup_socket_addr(host: &str, port: u16) -> Result<SocketAddr, Error> {
        let ips = lookup_host(host)
            .map_err(|e| Error::Network(format!("Failed to find IP addresses: {:#?}", e)))?;

        let first_ip = ips
            .first()
            .ok_or_else(|| Error::Network("No IP addresses found".to_string()))?;

        Ok((*first_ip, port).into())
    }
}
