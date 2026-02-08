//! # spider-downloader
//!
//! Traits and implementations for HTTP downloaders in the `spider-lib` framework.
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
//!     async fn download(&self, request: Request) -> Result<Response, SpiderError> {
//!         todo!()
//!     }
//!     fn client(&self) -> &Self::Client {
//!         &self.client
//!     }
//! }
//! ```

mod reqwest_client;
mod traits;

pub use reqwest_client::ReqwestClientDownloader;
pub use traits::{Downloader, SimpleHttpClient};
