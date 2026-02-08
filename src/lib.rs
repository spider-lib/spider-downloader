//! # spider-downloader
//!
//! Provides traits and implementations for HTTP downloaders in the `spider-lib` framework.
//!
//! ## Overview
//!
//! The `spider-downloader` crate defines the foundational traits for handling
//! HTTP requests and responses within the web crawling framework. It abstracts
//! the underlying HTTP client implementation, allowing for flexibility in
//! choosing different HTTP libraries while maintaining a consistent interface
//! for the crawling engine.
//!
//! ## Key Components
//!
//! - **Downloader Trait**: Interface for components that execute web requests
//!   and produce `Response` objects. Implementations typically wrap HTTP client
//!   libraries like `reqwest`.
//! - **SimpleHttpClient Trait**: Basic interface for performing simple GET
//!   requests, used for internal utility functions or when a full `Request`
//!   object isn't necessary.
//!
//! ## Architecture
//!
//! The downloader system is designed to be pluggable, allowing users to
//! implement custom downloaders with different behaviors (e.g., with different
//! retry strategies, proxy support, or caching mechanisms).
//!
//! ## Example
//!
//! ```rust,ignore
//! use spider_downloader::{Downloader, SimpleHttpClient};
//! use spider_util::{request::Request, response::Response, error::SpiderError};
//! use async_trait::async_trait;
//!
//! struct MyDownloader {
//!     client: reqwest::Client,
//! }
//!
//! #[async_trait]
//! impl Downloader for MyDownloader {
//!     type Client = reqwest::Client;
//!
//!     async fn download(&self, request: Request) -> Result<Response, SpiderError> {
//!         // Implementation for downloading web pages
//!         todo!()
//!     }
//!
//!     fn client(&self) -> &Self::Client {
//!         &self.client
//!     }
//! }
//! ```

use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use spider_util::error::SpiderError;
use spider_util::request::Request;
use spider_util::response::Response;
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
}
