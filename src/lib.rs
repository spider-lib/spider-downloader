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
//! - **ReqwestClientDownloader**: Default implementation of the Downloader trait using reqwest.
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

mod reqwest_client;
mod traits;

pub use reqwest_client::ReqwestClientDownloader;
pub use traits::{Downloader, SimpleHttpClient};
