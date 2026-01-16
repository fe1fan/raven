//! 简单的 HTTP 请求/响应解析器
//!
//! 使用标准库实现，无外部依赖。

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

/// HTTP 请求
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// 从 TcpStream 解析 HTTP 请求
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, String> {
        let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);

        // 解析请求行
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|e| e.to_string())?;

        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid request line".to_string());
        }

        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        // 解析 headers
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).map_err(|e| e.to_string())?;
            let line = line.trim();

            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        // 解析 body
        let mut body = Vec::new();
        if let Some(content_length) = headers.get("content-length") {
            if let Ok(len) = content_length.parse::<usize>() {
                body.resize(len, 0);
                reader.read_exact(&mut body).map_err(|e| e.to_string())?;
            }
        }

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    /// 构建完整 URL
    pub fn url(&self, host: &str) -> String {
        format!("http://{}{}", host, self.path)
    }

    /// 获取 body 文本
    pub fn body_text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

/// HTTP 响应
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self::new(200, "OK")
    }
}

impl HttpResponse {
    pub fn new(status: u16, status_text: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());

        Self {
            status,
            status_text: status_text.to_string(),
            headers,
            body: Vec::new(),
        }
    }

    pub fn ok(body: &str) -> Self {
        let mut resp = Self::new(200, "OK");
        resp.body = body.as_bytes().to_vec();
        resp.headers.insert(
            "content-length".to_string(),
            resp.body.len().to_string(),
        );
        resp
    }

    pub fn error(status: u16, message: &str) -> Self {
        let status_text = match status {
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Error",
        };

        let mut resp = Self::new(status, status_text);
        resp.body = message.as_bytes().to_vec();
        resp.headers.insert(
            "content-length".to_string(),
            resp.body.len().to_string(),
        );
        resp
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_lowercase(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.headers
            .insert("content-length".to_string(), body.len().to_string());
        self.body = body;
        self
    }

    /// 将响应写入 TcpStream
    pub fn write_to(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        // 状态行
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status, self.status_text);
        stream.write_all(status_line.as_bytes())?;

        // Headers
        for (key, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            stream.write_all(header_line.as_bytes())?;
        }

        // 空行
        stream.write_all(b"\r\n")?;

        // Body
        stream.write_all(&self.body)?;
        stream.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_response() {
        let resp = HttpResponse::ok("Hello World")
            .with_header("X-Custom", "value");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, b"Hello World");
        assert_eq!(resp.headers.get("x-custom"), Some(&"value".to_string()));
    }
}
