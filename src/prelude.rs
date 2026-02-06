//! A "prelude" for users of the `spider-lib` crate.
//!
//! This prelude re-exports the most commonly used traits, structs, and macros
//! from the entire spider framework so that they can be easily imported.
//!
//! # Example
//!
//! ```
//! use spider_lib::prelude::*;
//! ```

pub use spider_core::{
    // Core structs
    Crawler,
    CrawlerBuilder,
    // Core traits
    Downloader,
    Middleware,
    ParseOutput,
    Pipeline,
    // Core errors
    PipelineError,
    Request,
    Response,
    ScrapedItem,
    Spider,
    SpiderError,
    // Essential re-exports for trait implementation
    async_trait,
    // Core modules
    scheduler::Scheduler,
    stats::StatCollector,
    state::CrawlerState,
};
pub use spider_core::spider::Spider;
pub use spider_macro::scraped_item;

pub use spider_middlewares::{
    rate_limit::RateLimitMiddleware,
    referer::RefererMiddleware,
    retry::RetryMiddleware,
    user_agent::UserAgentMiddleware,
    cookies::CookieMiddleware,
    http_cache::HttpCacheMiddleware,
    robots_txt::RobotsTxtMiddleware,
    proxy::ProxyMiddleware,
};

pub use spider_pipelines::{
    console_writer::ConsoleWriterPipeline,
    deduplication::DeduplicationPipeline,
    csv_exporter::CsvExporterPipeline,
    json_writer::JsonWriterPipeline,
    jsonl_writer::JsonlWriterPipeline,
    sqlite_writer::SqliteWriterPipeline,
};

pub use tokio;
pub use dashmap::DashMap;
