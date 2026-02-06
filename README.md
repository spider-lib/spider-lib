# spider-lib

A Rust-based web scraping framework inspired by Scrapy.

[![crates.io](https://img.shields.io/crates/v/spider-lib.svg)](https://crates.io/crates/spider-lib)
[![docs.rs](https://docs.rs/spider-lib/badge.svg)](https://docs.rs/spider-lib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`spider-lib` is an asynchronous, concurrent web scraping library for Rust. It's designed to be a lightweight yet powerful tool for building and running scrapers for projects of any size. If you're familiar with Scrapy's architecture of Spiders, Middlewares, and Pipelines, you'll feel right at home.

This is the main entry point for the spider framework, integrating the core engine (spider-core), macro utilities (spider-macro), middleware components (spider-middlewares), and pipeline components (spider-pipelines) into a unified, easy-to-use library.

## Getting Started

To use `spider-lib`, add it to your project's `Cargo.toml`:

```toml
[dependencies]
spider-lib = "0.2" # Check crates.io for the latest version
```

## Quick Example

Here's a minimal example of a spider that scrapes quotes from `quotes.toscrape.com`.

For convenience, `spider-lib` offers a prelude that re-exports the most commonly used items.

```rust
// Use the prelude for easy access to common types and traits.
use spider_lib::prelude::*;
use spider_lib::utils::ToSelector; // ToSelector is not in the prelude

#[scraped_item]
pub struct QuoteItem {
    pub text: String,
    pub author: String,
}

pub struct QuotesSpider;

#[async_trait]
impl Spider for QuotesSpider {
    type Item = QuoteItem;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["http://quotes.toscrape.com/"]
    }

    async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
        let html = response.to_html()?;
        let mut output = ParseOutput::new();

        for quote in html.select(&".quote".to_selector()?) {
            let text = quote.select(&".text".to_selector()?).next().map(|e| e.text().collect()).unwrap_or_default();
            let author = quote.select(&".author".to_selector()?).next().map(|e| e.text().collect()).unwrap_or_default();
            output.add_item(QuoteItem { text, author });
        }

        if let Some(next_href) = html.select(&".next > a[href]".to_selector()?).next().and_then(|a| a.attr("href")) {
            let next_url = response.url.join(next_href)?;
            output.add_request(Request::new(next_url));
        }

        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // The builder defaults to using ReqwestClientDownloader
    let crawler = CrawlerBuilder::new(QuotesSpider)
        .build()
        .await?;

    crawler.start_crawl().await?;

    Ok(())
}
```

## Features

*   **Asynchronous & Concurrent:** `spider-lib` provides a high-performance, asynchronous web scraping framework built on `tokio`, leveraging an actor-like concurrency model for efficient task handling.
*   **Crawl Statistics:** Automatically collects and logs comprehensive statistics about the crawl's progress, including requests, responses (with status codes), items scraped, and downloaded bytes. The `StatCollector` can also be accessed programmatically via `crawler.get_stats()` for custom reporting and integration.
*   **Graceful Shutdown:** Ensures clean termination on `Ctrl+C`, allowing in-flight tasks to complete and flushing all data.
*   **Checkpoint and Resume:** Allows saving the crawler's state (scheduler, pipelines) to a file and resuming the crawl later, supporting both manual and periodic automatic saves. This includes salvaging un-processed requests.
*   **Request Deduplication:** Utilizes request fingerprinting to prevent duplicate requests from being processed, ensuring efficiency and avoiding redundant work. Now enhanced with Bloom Filter technology for faster duplicate detection.
*   **Familiar Architecture:** Leverages a modular design with Spiders, Middlewares, and Item Pipelines, drawing inspiration from Scrapy.
*   **Configurable Concurrency:** Offers fine-grained control over the number of concurrent downloads, parsing workers, and pipeline processing for optimized performance.
*   **Advanced Link Extraction:** Includes a powerful `Response` object method to comprehensively extract, resolve, and categorize various types of links from HTML content.
*   **Fluent Configuration:** A `CrawlerBuilder` API simplifies the assembly and configuration of your web crawler.
*   **Efficient URL Visitation Tracking:** Uses a combination of Bloom Filter and LRU cache to quickly check if URLs have been visited, making it much faster when dealing with millions of URLs.
*   **Adaptive Batching:** Automatically adjusts batch sizes and delays based on how busy the scheduler is, helping optimize throughput without overwhelming target servers.
*   **Optimized Middleware Processing:** Reduces contention and speeds up processing with a shared middleware manager that handles concurrent access efficiently.

For complete, runnable examples, please refer to the `examples/` directory in this repository. You can run an example using `cargo run --example <example_name> --features <features>`, for instance: `cargo run --example quotes --features "pipeline-json"`.

## Configuration Examples

While `spider-lib` provides sensible defaults, you can finely tune its behavior by configuring middlewares, pipelines, and the crawler itself.

### Middlewares

Middlewares inspect and modify requests and responses. They can be added to the `CrawlerBuilder`.

The following middlewares are included by default:
*   **Rate Limiting:** Controls request rates to prevent server overload.
*   **Retries:** Automatically retries failed or timed-out requests.
*   **User-Agent Rotation:** Manages and rotates user agents.
*   **Referer Management:** Handles the `Referer` header.

Additional middlewares are available via feature flags:
*   **Cookie Management:** Persists cookies across requests to maintain sessions (`middleware-cookies`).
*   **HTTP Caching:** Caches responses to accelerate development (`middleware-http-cache`).
*   **Respect Robots.txt:** Adheres to `robots.txt` rules (`middleware-robots-txt`).
*   **Proxy Rotation:** Manages and rotates proxy servers (`middleware-proxy`).

#### `CookieMiddleware`

This middleware automatically manages cookies to maintain sessions across requests, which is essential for scraping sites that require logins. It is enabled via the `middleware-cookies` feature. For robust operation, it's also integrated with the checkpointing system, so cookie sessions are saved and restored along with the rest of the crawl state.

**Basic Usage**

The simplest way to use the middleware is to add it to the crawler.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `middleware-cookies` feature in Cargo.toml
use spider_lib::middlewares::cookies::CookieMiddleware;

// ... inside your main async function
let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    .add_middleware(CookieMiddleware::new())
    .build()
    .await?;
```

**Loading Cookies from a File**

For spiders that need to start with a pre-existing session (e.g., being logged in), you can load cookies from a file. The middleware supports several formats. This is an asynchronous operation and should be awaited.

-   **`from_netscape_file`**: Loads cookies from a Netscape-style cookie jar file.
-   **`from_json`**: Loads cookies from a JSON file in the format used by the `cookie_store` crate.
-   **`from_rfc6265`**: Loads cookies from a file where each line is a raw `Set-Cookie` header.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `middleware-cookies` feature in Cargo.toml
use spider_lib::middlewares::cookies::CookieMiddleware;

// ... inside your main async function

// Load cookies from a file exported from your browser
let cookie_middleware = CookieMiddleware::from_netscape_file("path/to/your/cookies.txt")
    .await?;

let crawler = CrawlerBuilder::new(YourSpider)
    .add_middleware(cookie_middleware)
    .build()
    .await?;
```

#### `UserAgentMiddleware`

This middleware manages and rotates User-Agent strings. It can be configured with different rotation strategies, User-Agent sources, and even apply different rules for different domains.

**Available Strategies (`UserAgentRotationStrategy`):**
*   `Random`: (Default) Selects a User-Agent randomly.
*   `Sequential`: Cycles through the list of User-Agents in order.
*   `Sticky`: On first encounter, a User-Agent is "stuck" to a domain for the entire crawl.
*   `StickySession`: A User-Agent is "stuck" to a domain for a configured duration.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::middlewares::user_agent::{
    UserAgentMiddleware, UserAgentRotationStrategy, UserAgentSource, BuiltinUserAgentList
};
use std::time::Duration;

// ... inside your main async function
let ua_middleware = UserAgentMiddleware::builder()
    // Set the default strategy for all domains
    .strategy(UserAgentRotationStrategy::Random)
    // Set the default source of User-Agents
    .source(UserAgentSource::Builtin(BuiltinUserAgentList::Chrome))
    // Set the session duration for the `StickySession` strategy
    .session_duration(Duration::from_secs(60 * 5))
    // Use a different User-Agent source specifically for "example.org"
    .per_domain_source(
        "example.org".to_string(),
        UserAgentSource::Builtin(BuiltinUserAgentList::Firefox)
    )
    // Use a different strategy for "example.com"
    .per_domain_strategy(
        "example.com".to_string(),
        UserAgentRotationStrategy::Sticky
    )
    .build()?;
```

#### `RateLimitMiddleware`

This middleware controls the request rate to avoid overloading servers. By default, it uses an adaptive limiter on a per-domain basis. You can configure it to use a fixed rate instead.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::middlewares::rate_limit::{RateLimitMiddleware, Scope};

// ... inside your main async function
let rate_limit_middleware = RateLimitMiddleware::builder()
    // Apply one rate limit across all domains
    .scope(Scope::Global)
    // Use a token bucket algorithm to allow 5 requests per second
    .use_token_bucket_limiter(5)
    .build();
```

#### `HttpCacheMiddleware`

This middleware caches HTTP responses to disk, which can significantly speed up development and re-runs by avoiding redundant network requests. It's enabled via the `middleware-http-cache` feature.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `middleware-http-cache` feature in Cargo.toml
use spider_lib::middlewares::http_cache::HttpCacheMiddleware;
use std::path::PathBuf;

// ... inside your main async function
let http_cache_middleware = HttpCacheMiddleware::builder()
    // Set a custom directory for storing cache files
    .cache_dir(PathBuf::from("output/http_cache"))
    .build()?;
```

#### `RefererMiddleware`

This middleware automatically manages the `Referer` HTTP header, simulating natural browsing behavior.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::middlewares::referer::RefererMiddleware;

// ... inside your main async function
let referer_middleware = RefererMiddleware::new()
    // Ensure referer is only set for requests to the same origin
    .same_origin_only(true)
    // Keep a maximum of 500 referer URLs in memory
    .max_chain_length(500)
    // Do not include URL fragments in the referer header
    .include_fragment(false);
```

#### `RetryMiddleware`

This middleware automatically retries failed requests based on HTTP status codes or network errors, using an exponential backoff strategy.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::middlewares::retry::RetryMiddleware;
use std::time::Duration;

// ... inside your main async function
let retry_middleware = RetryMiddleware::new()
    // Allow up to 5 retry attempts
    .max_retries(5)
    // Define which HTTP status codes should trigger a retry
    .retry_http_codes(vec![500, 502, 503, 504, 408, 429])
    // Set the exponential backoff factor
    .backoff_factor(2.0)
    // Cap the maximum delay between retries at 300 seconds (5 minutes)
    .max_delay(Duration::from_secs(300));
```

#### `RobotsTxtMiddleware`

This middleware respects `robots.txt` rules, preventing the crawler from accessing disallowed paths. It's enabled via the `middleware-robots-txt` feature.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `middleware-robots-txt` feature in Cargo.toml
use spider_lib::middlewares::robots_txt::RobotsTxtMiddleware;
use std::time::Duration;

// ... inside your main async function
let robots_txt_middleware = RobotsTxtMiddleware::new()
    // Cache robots.txt rules for 12 hours
    .cache_ttl(Duration::from_secs(60 * 60 * 12))
    // Store up to 5000 robots.txt files in cache
    .cache_capacity(5_000)
    // Set a timeout of 10 seconds for fetching robots.txt files
    .request_timeout(Duration::from_secs(10));
```

#### `ProxyMiddleware`

This middleware manages and rotates proxy servers for outgoing requests. It's crucial for avoiding IP bans and rate limiting by distributing requests across multiple IP addresses. It supports various rotation strategies and advanced detection of IP blocking. It is enabled via the `middleware-proxy` feature.

**Available Strategies (`ProxyRotationStrategy`):**
*   `Sequential`: (Default) Cycles through the list of proxies in order.
*   `Random`: Selects a proxy randomly from the available pool.
*   `StickyFailover`: Uses one proxy until a failure is detected (based on status code or body content), then rotates to the next.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `middleware-proxy` feature in Cargo.toml
use spider_lib::middlewares::proxy::{ProxyMiddleware, ProxySource, ProxyRotationStrategy};
use std::path::PathBuf;

// ... inside your main async function
let proxy_middleware = ProxyMiddleware::builder()
    // Load proxies from a file (one URL per line)
    .source(ProxySource::File(PathBuf::from("proxies.txt")))
    // Use the StickyFailover strategy
    .strategy(ProxyRotationStrategy::StickyFailover)
    // Configure texts to detect in the response body that indicate an IP block
    .with_block_detection_texts(vec![
        "Your IP has been blocked".to_string(),
        "Please complete the CAPTCHA".to_string(),
        "Access Denied".to_string(),
    ])
    .build()?;

let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    .add_middleware(proxy_middleware)
    // ... configure other middlewares and pipelines
    .build()
    .await?;
```

### Pipelines

Item Pipelines are used for processing, filtering, or saving scraped items.

The following pipelines are included by default:
*   **Deduplication:** Filters out duplicate items based on a configurable key.
*   **Console Writer:** A simple pipeline for printing items to the console.

Exporter pipelines are available via feature flags:
*   **JSON / JSON Lines:** Saves items to `.json` or `.jsonl` files (`pipeline-json`).
*   **CSV:** Saves items to `.csv` files (`pipeline-csv`).
*   **SQLite:** Saves items to a SQLite database (`pipeline-sqlite`).

#### `ConsoleWriterPipeline`

A simple pipeline that prints each scraped item to the console. Useful for debugging.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::pipelines::console_writer::ConsoleWriterPipeline;

// ... inside your main async function
let console_pipeline = ConsoleWriterPipeline::new();
```

#### `DeduplicationPipeline`

This pipeline filters out duplicate items based on a configurable set of fields.

```rust,no_run
use spider_lib::prelude::*;
use spider_lib::pipelines::deduplication::DeduplicationPipeline;

// ... inside your main async function
let deduplication_pipeline = DeduplicationPipeline::new(&["url", "title"]);
```

#### `JsonWriterPipeline` & `JsonlWriterPipeline`

These pipelines save scraped items to a file. They are enabled with the `pipeline-json` feature.
*   `JsonWriterPipeline`: Collects all items and writes them to a single, pretty-printed JSON array at the end of the crawl.
*   `JsonlWriterPipeline`: Writes each item as a separate JSON object on a new line, which is efficient for streaming large amounts of data.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `pipeline-json` feature in Cargo.toml
use spider_lib::pipelines::json_writer::JsonWriterPipeline;
use spider_lib::pipelines::jsonl_writer::JsonlWriterPipeline;

// ... inside your main async function
let json_pipeline = JsonWriterPipeline::new("output/items.json")?;
let jsonl_pipeline = JsonlWriterPipeline::new("output/items.jsonl")?;

let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    .add_pipeline(json_pipeline)
    .add_pipeline(jsonl_pipeline)
    // ... configure other middlewares
    .build()
    .await?;
```

#### `CsvExporterPipeline`

This pipeline saves items to a CSV file, enabled with the `pipeline-csv` feature. The CSV headers are automatically inferred from the fields of the first item scraped.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `pipeline-csv` feature in Cargo.toml
use spider_lib::pipelines::csv_exporter::CsvExporterPipeline;

// ... inside your main async function
let csv_pipeline = CsvExporterPipeline::new("output/items.csv")?;

let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    .add_pipeline(csv_pipeline)
    // ... configure other middlewares
    .build()
    .await?;
```

#### `SqliteWriterPipeline`

This pipeline saves items to a SQLite database, enabled with the `pipeline-sqlite` feature. The table schema is automatically inferred from the fields of the first item scraped.

```rust,no_run
use spider_lib::prelude::*;
// Make sure to enable the `pipeline-sqlite` feature in Cargo.toml
use spider_lib::pipelines::sqlite_writer::SqliteWriterPipeline;

// ... inside your main async function
let sqlite_pipeline = SqliteWriterPipeline::new("output/items.db", "scraped_data")?;

let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    .add_pipeline(sqlite_pipeline)
    // ... configure other middlewares
    .build()
    .await?;
```

### Crawler Settings

You can configure the core behavior of the crawler, such as concurrency and checkpointing.

#### Checkpointing & Resuming

This feature allows a crawl to be paused and resumed later. When the crawler starts, it will load the state from the checkpoint file if it exists. This feature is enabled by the `checkpoint` flag.

```rust,no_run
use spider_lib::prelude::*;
use std::time::Duration;

// ... inside your main async function
let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    // Set the path to save/load the checkpoint file
    .with_checkpoint_path("output/my_crawl.checkpoint")
    // Automatically save the state every 10 minutes
    .with_checkpoint_interval(Duration::from_secs(60 * 10))
    // ... configure your other middlewares, and pipelines
    .build()
    .await?;
```

#### Concurrency

You can control the parallelism of different parts of the crawl to manage system resources and target server load.

```rust,no_run
use spider_lib::prelude::*;

// ... inside your main async function
let crawler = CrawlerBuilder::new(YourSpider) // Assumes `YourSpider` is a defined Spider
    // Set the maximum number of concurrent downloads
    .max_concurrent_downloads(10)
    // Set the number of CPU workers for parsing responses
    .max_parser_workers(4)
    // Set the maximum number of items to be processed by pipelines concurrently
    .max_concurrent_pipelines(20)
    // ... configure your other middlewares, and pipelines
    .build()
    .await?;
```

#### Efficiency Optimizations

The crawler includes several built-in optimizations for better performance:

**Bloom Filter Integration**: The scheduler now uses a Bloom Filter for quick preliminary checks of visited URLs, cutting down the computational overhead for duplicate detection when handling millions of URLs.

**Adaptive Batching**: The system automatically adjusts batch sizes and delays based on how busy the scheduler is, optimizing throughput without overwhelming target servers.

**Optimized Middleware Processing**: A shared middleware manager reduces contention and speeds up processing by handling concurrent access more efficiently.


## All Features Included

`spider-lib` now includes all functionality by default, making it easier to use without needing to manage feature flags. All middlewares, pipelines, and core features are available immediately after adding the dependency.

All of the following are now available without any additional configuration:

| Component Type | Components | Description |
| :--- | :--- | :--- |
| **Pipelines** | | |
| | `ConsoleWriterPipeline`, `DeduplicationPipeline` | Basic pipelines for debugging and deduplication. |
| | `JsonWriterPipeline`, `JsonlWriterPipeline` | Saves items to `.json` or `.jsonl` files. |
| | `CsvExporterPipeline` | Saves items to a `.csv` file. |
| | `SqliteWriterPipeline` | Saves items to a SQLite database. |
| **Middlewares** | | |
| | `CookieMiddleware` | Manages cookies and sessions across requests. |
| | `HttpCacheMiddleware` | Caches HTTP responses to disk to speed up development. |
| | `RobotsTxtMiddleware` | Respects `robots.txt` rules for websites. |
| | `UserAgentMiddleware` | Rotates User-Agent strings to avoid detection. |
| | `RateLimitMiddleware` | Controls request rates to prevent server overload. |
| | `RetryMiddleware` | Automatically retries failed requests. |
| | `RefererMiddleware` | Manages the `Referer` HTTP header. |
| | `ProxyMiddleware` | Manages and rotates proxy servers for outgoing requests. |
| **Core** | | |
| | Checkpointing System | Enables saving and resuming crawl state. |

Simply add spider-lib to your `Cargo.toml` and all functionality is ready to use:

```toml
[dependencies]
spider-lib = "0.5"
```
