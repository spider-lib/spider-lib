//! # spider-lib
//!
//! A Rust-based web scraping framework inspired by Scrapy.
//!
//! `spider-lib` is an asynchronous, concurrent web scraping library for Rust.
//! It's designed to be a lightweight yet powerful tool for building and running
//! scrapers for projects of any size. If you're familiar with Scrapy's architecture
//! of Spiders, Middlewares, and Pipelines, you'll feel right at home.
//!
//! This is the main entry point for the spider framework, integrating the core engine
//! (spider-core), macro utilities (spider-macro), middleware components (spider-middlewares),
//! and pipeline components (spider-pipelines) into a unified, easy-to-use library.
//!
//! ## Getting Started
//!
//! To use `spider-lib`, add it to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! spider-lib = "0.5"
//! ```
//!
//! ## Quick Example
//!
//! Here's a minimal example of a spider that scrapes quotes from `quotes.toscrape.com`:
//!
//! ```rust,ignore
//! use spider_lib::prelude::*;
//! use spider_core::utils::ToSelector;
//!
//! #[spider_lib::scraped_item]
//! #[derive(Default)]
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
//!
//!     fn start_urls(&self) -> Vec<&'static str> {
//!         vec!["http://quotes.toscrape.com/"]
//!     }
//!
//!     async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
//!         let html = response.to_html()?;
//!         let mut output = ParseOutput::new();
//!
//!         for quote in html.select(&".quote".to_selector()?) {
//!             let text = quote.select(&".text".to_selector()?).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
//!             let author = quote.select(&".author".to_selector()?).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
//!             output.add_item(Quote { text, author });
//!         }
//!
//!         if let Some(next_href) = html.select(&".next > a[href]".to_selector()?).next().and_then(|a| a.attr("href")) {
//!             let next_url = response.url.join(next_href)?;
//!             output.add_request(Request::new(next_url));
//!         }
//!
//!         Ok(output)
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SpiderError> {
//!     let crawler = CrawlerBuilder::new(QuotesSpider)
//!         .build()
//!         .await?;
//!
//!     crawler.start_crawl().await?;
//!     Ok(())
//! }
//! ```

pub mod prelude;

// Re-export everything from sub-crates through the prelude
pub use prelude::*;
