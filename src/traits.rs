//! Traits for HTTP downloaders in the `spider-lib` framework.

use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use spider_util::error::SpiderError;
use spider_util::request::Request;
use spider_util::response::Response;
#[cfg(feature = "stream")]
use spider_util::stream_response::StreamResponse;
use std::time::Duration;

/// A simple HTTP client trait for fetching web content.
#[async_trait]
pub trait SimpleHttpClient: Send + Sync {
    /// Fetches the content of a URL as text.
    async fn get_text(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<(StatusCode, Bytes), SpiderError>;
}

/// A trait for HTTP downloaders that can fetch web pages and apply middleware
#[async_trait]
pub trait Downloader: Send + Sync + 'static {
    type Client: Send + Sync;

    /// Download a web page using the provided request.
    /// This function focuses solely on executing the HTTP request.
    async fn download(&self, request: Request) -> Result<Response, SpiderError>;

    /// Returns a reference to the underlying HTTP client.
    fn client(&self) -> &Self::Client;

    /// Download a web page as a stream response (optional feature).
    #[cfg(feature = "stream")]
    async fn download_stream(&self, request: Request) -> Result<StreamResponse, SpiderError> {
        // Default implementation converts regular response to stream
        let response = self.download(request).await?;
        response
            .to_stream_response()
            .await
            .map_err(|e| SpiderError::IoError(e.to_string()))
    }
}

