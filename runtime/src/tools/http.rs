//! HTTP 工具

use anyhow::Result;

/// HTTP 请求
pub struct HttpRequest {
    pub url: String,
    pub method: String,
}

/// 执行 HTTP 请求
pub fn request(req: HttpRequest) -> Result<String> {
    tracing::debug!("HTTP request: {} {}", req.method, req.url);
    Ok("{}".to_string())
}
