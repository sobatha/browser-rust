use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        self.url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .nth(1) // Extract the part after the first '/'
            .map(|path| path.splitn(2, '?').next().unwrap_or("").to_string())
            .unwrap_or_else(String::new) // Default to empty string if no path
    }

    fn extract_searchpart(&self) -> String {
        self.url
            .trim_start_matches("http://")
            .splitn(2, '?')
            .nth(1) // Extract the part after the first '/'
            .map(|path| path.to_string())
            .unwrap_or_else(String::new)
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urls() {
        let mut url = Url::new("http://google.github.io:8080/comprehensive-rust/types-and-values/values?page=10".to_string());

        assert_eq!(url.parse().is_ok(), true);
        assert_eq!(url.host(), "google.github.io");
        assert_eq!(url.port(), "8080");
        assert_eq!(url.path(), "comprehensive-rust/types-and-values/values");
        assert_eq!(url.searchpart(), "page=10");
    }

    #[test]
    fn test_url() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port_path_searchpart() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_localhost() {
        let url = "http://localhost:8000".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "localhost".to_string(),
            port: "8000".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com:8888/index.html".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }
}