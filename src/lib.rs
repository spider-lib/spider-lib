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
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use dashmap::DashMap;
//!
//! #[scraped_item]
//! struct Quote {
//!     text: String,
//!     author: String,
//! }
//!
//! struct QuotesSpider;
//!
//! // State for tracking information during crawling
//! #[derive(Clone, Default)]
//! struct QuotesSpiderState {
//!     page_count: Arc<AtomicUsize>,
//!     visited_urls: Arc<DashMap<String, bool>>,
//! }
//!
//! impl QuotesSpiderState {
//!     fn increment_page_count(&self) {
//!         self.page_count.fetch_add(1, Ordering::SeqCst);
//!     }
//!     
//!     fn mark_url_visited(&self, url: String) {
//!         self.visited_urls.insert(url, true);
//!     }
//! }
//!
//! #[async_trait]
//! impl Spider for QuotesSpider {
//!     type Item = Quote;
//!     type State = QuotesSpiderState;
//!
//!     fn start_urls(&self) -> Vec<&'static str> {
//!         vec!["http://quotes.toscrape.com/"]
//!     }
//!
//!     async fn parse(&self, response: Response, state: &Self::State) -> Result<ParseOutput<Self::Item>, SpiderError> {
//!         // Update state - can be done concurrently without blocking the spider
//!         state.increment_page_count();
//!         state.mark_url_visited(response.url.to_string());
//!         
//!         // parsing logic
//!         todo!()
//!     }
//! }
//! ```
//!
//! **Note**: Notice that the `Spider` implementation now uses an immutable reference (`&self`)
//! and receives a separate state parameter (`state: &Self::State`). This enables more efficient
//! concurrent crawling by eliminating the need for mutex locks on the spider itself.

pub mod prelude;
pub use prelude::*;
