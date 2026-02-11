//! Reqwest-based Downloader implementation for the `spider-lib` framework.
//!
//! This module provides `ReqwestClientDownloader`, a concrete implementation
//! of the `Downloader` trait that leverages the `reqwest` HTTP client library.
//! It is responsible for executing HTTP requests defined by `Request` objects
//! and converting the received HTTP responses into `Response` objects suitable
//! for further processing by the crawler.
//!
//! This downloader handles various HTTP methods, request bodies (JSON, form data, bytes),
//! and integrates with the framework's error handling.

use crate::{Downloader, SimpleHttpClient};
use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use reqwest::{Client, Proxy};
use spider_util::error::SpiderError;
use spider_util::request::{Body, Request};
use spider_util::response::Response;
use std::time::Duration;
use log::info;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
impl SimpleHttpClient for Client {
    async fn get_text(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<(StatusCode, Bytes), SpiderError> {
        let resp = self.get(url).timeout(timeout).send().await?;
        let status = resp.status();
        let body = resp.bytes().await?;
        Ok((status, body))
    }
}

/// Concrete implementation of Downloader using reqwest client
pub struct ReqwestClientDownloader {
    client: Client,
    timeout: Duration,
    /// Per-host connection pools for better resource management
    host_clients: Arc<RwLock<HashMap<String, Client>>>,
}

#[async_trait]
impl Downloader for ReqwestClientDownloader {
    type Client = Client;

    /// Returns a reference to the underlying HTTP client.
    fn client(&self) -> &Self::Client {
        &self.client
    }

    async fn download(&self, request: Request) -> Result<Response, SpiderError> {
        info!(
            "Downloading {} (fingerprint: {})",
            request.url,
            request.fingerprint()
        );

        let Request {
            url,
            method,
            headers,
            body,
            meta,
            ..
        } = request;

        // Get host-specific client if available, otherwise use default
        let host = url.host_str().unwrap_or("").to_string();
        // Convert DashMap to HashMap for the host client creation
        let meta_hashmap: std::collections::HashMap<String, serde_json::Value> = 
            meta.iter().map(|entry| (entry.key().clone().into_owned(), entry.value().clone())).collect();
        let mut client_to_use = self.get_or_create_host_client(&host, &meta_hashmap).await;

        if let Some(proxy_val) = meta.get("proxy")
            && let Some(proxy_str) = proxy_val.as_str()
        {
            match Proxy::all(proxy_str) {
                Ok(proxy) => {
                    let new_client = Client::builder()
                        .timeout(self.timeout)
                        .proxy(proxy)
                        .build()
                        .map_err(|e| SpiderError::ReqwestError(e.into()))?;
                    client_to_use = new_client;
                }
                Err(e) => {
                    return Err(SpiderError::ReqwestError(e.into()));
                }
            }
        }

        let mut req_builder = client_to_use.request(method, url.clone());

        if let Some(body_content) = body {
            req_builder = match body_content {
                Body::Json(json_val) => req_builder.json(&json_val),
                Body::Form(form_val) => {
                    let mut form_map = std::collections::HashMap::new();
                    for entry in form_val.iter() {
                        form_map.insert(entry.key().clone(), entry.value().clone());
                    }
                    req_builder.form(&form_map)
                }
                Body::Bytes(bytes_val) => req_builder.body(bytes_val),
            };
        }

        let res = req_builder.headers(headers).send().await?;

        let response_url = res.url().clone();
        let status = res.status();
        let response_headers = res.headers().clone();
        let response_body = res.bytes().await?;

        Ok(Response {
            url: response_url,
            status,
            headers: response_headers,
            body: response_body,
            request_url: url,
            meta,
            cached: false,
        })
    }
}

impl ReqwestClientDownloader {
    /// Creates a new `ReqwestClientDownloader` with a default timeout of 30 seconds.
    pub fn new() -> Self {
        Self::new_with_timeout(Duration::from_secs(30))
    }

    /// Creates a new `ReqwestClientDownloader` with a specified request timeout.
    pub fn new_with_timeout(timeout: Duration) -> Self {
        let base_client = Client::builder()
            .timeout(timeout)
            .pool_max_idle_per_host(200)
            .pool_idle_timeout(Duration::from_secs(120))
            .tcp_keepalive(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap();
            
        ReqwestClientDownloader {
            client: base_client.clone(),
            timeout,
            host_clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets or creates a host-specific client with optimized settings for that host
    async fn get_or_create_host_client(&self, host: &str, _meta: &std::collections::HashMap<String, serde_json::Value>) -> Client {
        {
            let clients = self.host_clients.read().await;
            if let Some(client) = clients.get(host) {
                return client.clone();
            }
        }

        // Create a new client for this host with optimized settings
        let host_specific_client = Client::builder()
            .timeout(self.timeout)
            .pool_max_idle_per_host(50) // Smaller pool per host to distribute connections
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_keepalive(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        {
            let mut clients = self.host_clients.write().await;
            // Double-check pattern to avoid race condition
            if let Some(client) = clients.get(host) {
                return client.clone();
            }
            clients.insert(host.to_string(), host_specific_client.clone());
        }

        host_specific_client
    }
}

impl Default for ReqwestClientDownloader {
    fn default() -> Self {
        Self::new()
    }
}
