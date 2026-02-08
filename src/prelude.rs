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
    ReqwestClientDownloader,
    Spider,
    // Essential re-exports for trait implementation
    async_trait,
    // Core modules
    scheduler::Scheduler,
    state::CrawlerState,
    stats::StatCollector,
    tokio,
};

// Re-export ParseOutput and ScrapedItem from spider_util
pub use spider_util::item::{ParseOutput, ScrapedItem};

// Re-export Pipeline from spider_pipeline
pub use spider_pipeline::pipeline::Pipeline;

// Import types from other crates
pub use spider_macro::scraped_item;
pub use spider_middleware::middleware::{Middleware, MiddlewareAction};
pub use spider_util::{
    error::{PipelineError, SpiderError},
    request::Request,
    response::Response,
    serde, serde_json,
    utils::{is_same_site, normalize_origin, validate_output_dir, create_dir, ToSelector},
};

pub use spider_middleware::{
    cookies::CookieMiddleware, http_cache::HttpCacheMiddleware, proxy::ProxyMiddleware,
    rate_limit::RateLimitMiddleware, referer::RefererMiddleware, retry::RetryMiddleware,
    robots_txt::RobotsTxtMiddleware, user_agent::UserAgentMiddleware,
};

pub use spider_pipeline::{
    console_writer::ConsoleWriterPipeline, csv_exporter::CsvExporterPipeline,
    deduplication::DeduplicationPipeline, json_writer::JsonWriterPipeline,
    jsonl_writer::JsonlWriterPipeline, sqlite_writer::SqliteWriterPipeline,
    streaming_json_writer::StreamingJsonWriterPipeline,
};
