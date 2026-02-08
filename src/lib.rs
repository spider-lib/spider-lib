//! # spider-lib
//!
//! A Rust-based web scraping framework inspired by Scrapy.
//!
//! `spider-lib` is an asynchronous web scraping library for Rust.
//! It integrates core engine, macros, middleware, and pipelines into a unified library.
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! spider-lib = "1.1.1"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! ```rust,ignore
//! use spider_lib::prelude::*;
//!
//! #[scraped_item]
//! struct Quote {
//!     text: String,
//!     author: String,
//! }
//!
//! struct QuotesSpider;
//!
//! #[async_trait]
//! impl Spider for QuotesSpider {
//!     type Item = Quote;
//!     fn start_urls(&self) -> Vec<&'static str> {
//!         vec!["http://quotes.toscrape.com/"]
//!     }
//!     async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
//!         // parsing logic
//!         todo!()
//!     }
//! }
//! ```

pub mod prelude;
pub use prelude::*;
