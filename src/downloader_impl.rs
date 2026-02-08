//! Default implementation of the Downloader trait using reqwest.

use crate::{Downloader, SimpleHttpClient};
use async_trait::async_trait;
use http::StatusCode;
use reqwest::Client;
use spider_util::error::SpiderError;
use spider_util::request::Request;
use spider_util::response::Response;
use std::time::Duration;

/// A default downloader implementation using reqwest.
pub struct ReqwestClientDownloader {
    client: Client,
}

impl Default for ReqwestClientDownloader {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Downloader for ReqwestClientDownloader {
    type Client = Client;

    async fn download(&self, request: Request) -> Result<Response, SpiderError> {
        let mut req_builder = self.client.request(
            request.method.clone(),
            reqwest::Url::parse(&request.url.to_string()).map_err(|e| {
                SpiderError::UrlParseError(format!("Failed to parse URL: {}", e))
            })?,
        );

        // Add headers from the request
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone().into_bytes());
        }

        let response = req_builder.send().await.map_err(|e| {
            SpiderError::ReqwestError(e.to_string())
        })?;

        // Convert the response to our Response type
        let status = StatusCode::from_u16(response.status().as_u16()).map_err(|_| {
            SpiderError::StatusCodeError("Invalid status code".to_string())
        })?;

        let headers = response.headers().clone();
        
        let body = response.bytes().await.map_err(|e| {
            SpiderError::IoError(e.to_string())
        })?;

        let url = response.url().clone();

        Ok(Response {
            url,
            status,
            headers,
            body: body.to_vec(),
        })
    }

    fn client(&self) -> &Self::Client {
        &self.client
    }
}

#[async_trait]
impl SimpleHttpClient for Client {
    async fn get_text(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<(StatusCode, bytes::Bytes), SpiderError> {
        let response = self
            .get(url)
            .timeout(timeout)
            .send()
            .await
            .map_err(|e| SpiderError::ReqwestError(e.to_string()))?;

        let status = StatusCode::from_u16(response.status().as_u16()).map_err(|_| {
            SpiderError::StatusCodeError("Invalid status code".to_string())
        })?;

        let body = response.bytes().await.map_err(|e| {
            SpiderError::IoError(e.to_string())
        })?;

        Ok((status, body))
    }
}