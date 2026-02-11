# spider-lib

A Rust-based web scraping framework inspired by Scrapy (Python).

## Overview

`spider-lib` is a comprehensive web scraping framework written in Rust that takes inspiration from the popular Python Scrapy framework. It provides a robust, efficient, and extensible platform for building web crawlers and scrapers with features like asynchronous processing, middleware support, and flexible data pipelines.

## Features

- **Asynchronous Processing**: Built with async/await for efficient concurrent crawling
- **Modular Architecture**: Clean separation of concerns with multiple specialized crates
- **Middleware Support**: Extensible middleware system for customizing request/response handling
- **Flexible Pipelines**: Multiple output options for scraped data (JSON, CSV, SQLite, etc.)
- **Rate Limiting**: Built-in rate limiting to respect website policies
- **Retry Mechanisms**: Automatic retry for failed requests
- **User-Agent Rotation**: Automatic rotation of user agents
- **Cookie Management**: Persistent cookie handling across requests
- **HTTP Caching**: Development-friendly caching capabilities
- **Robots.txt Compliance**: Automatic adherence to robots.txt rules
- **Proxy Support**: Configurable proxy server usage
- **Deduplication**: Built-in duplicate detection and filtering

## Architecture

The framework is organized into several specialized crates:

### spider-core
The core engine that provides the fundamental components for building web scrapers, including the main `Crawler`, `Scheduler`, and `Spider` trait. It manages the flow of requests and responses and coordinates concurrent operations.

### spider-downloader
Provides traits and implementations for HTTP downloaders, abstracting the underlying HTTP client implementation for flexibility.

### spider-macro
Contains procedural macros that reduce boilerplate code, particularly the `#[scraped_item]` macro for defining data structures for scraped content.

### spider-middleware
Includes a comprehensive collection of middleware implementations that extend crawler functionality, such as rate limiting, retries, user-agent rotation, and cookie management.

### spider-pipeline
Provides built-in pipeline implementations for processing, filtering, transforming, and storing scraped data in various formats (JSON, CSV, SQLite, etc.).

### spider-util
Contains fundamental data structures, error types, and utility functions shared across all components of the framework.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
spider-lib = "1.1.1"
serde = { version = "1.0", features = ["derive"] }  # Required for #[scraped_item] macro
serde_json = "1.0"  # Required for #[scraped_item] macro
```

**Note**: When using the `#[scraped_item]` macro, you must also include `serde` and `serde_json` as direct dependencies in your project, as the macro generates code that references these crates directly.

### Features

Spider-lib provides optional features for specific functionality:

#### Middleware Features
- `middleware-cache` - Enable HTTP caching capabilities for development
- `middleware-proxy` - Enable proxy rotation functionality
- `middleware-user-agent` - Enable user-agent rotation
- `middleware-robots` - Enable robots.txt compliance checking
- `middleware-cookies` - Enable cookie management

#### Pipeline Features
- `pipeline-csv` - Enable CSV export functionality
- `pipeline-json` - Enable JSON writing functionality
- `pipeline-jsonl` - Enable JSONL writing functionality
- `pipeline-sqlite` - Enable SQLite database functionality
- `pipeline-stream-json` - Enable stream JSON functionality

#### Core Features
- `checkpoint` - Enable checkpoint and resume functionality
- `cookie-store` - Enable advanced cookie store integration (Note: When using `middleware-cookies`, `cookie-store` should also be enabled)

#### Important Feature Relationships
- `middleware-cookies` and `cookie-store` are interdependent: When using `middleware-cookies`, `cookie-store` should also be enabled for full functionality
- When using `cookie-store`, `middleware-cookies` functionality may be desired for managing cookies effectively

By default, only core functionality is included. You can enable specific features as needed:

```toml
[dependencies]
spider-lib = { version = "0.5.1", features = ["middleware-cache", "pipeline-csv"] }
```

Or disable default features and enable only what you need:

```toml
[dependencies]
spider-lib = { version = "0.5.1", default-features = false, features = ["core"] }
```

## Usage

Here's a basic example of how to use the framework:

```rust
use spider_lib::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;

#[derive(Default)]
struct MySpider;

// State untuk tracking informasi selama crawling
#[derive(Clone, Default)]
struct MySpiderState {
    page_count: Arc<AtomicUsize>,
    visited_urls: Arc<DashMap<String, bool>>,
}

impl MySpiderState {
    fn increment_page_count(&self) {
        self.page_count.fetch_add(1, Ordering::SeqCst);
    }
    
    fn mark_url_visited(&self, url: String) {
        self.visited_urls.insert(url, true);
    }
}

#[scraped_item]
struct MyItem {
    title: String,
    url: String,
}

#[async_trait]
impl Spider for MySpider {
    type Item = MyItem;
    type State = MySpiderState;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://example.com"]
    }

    async fn parse(&self, response: Response, state: &Self::State) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Update state - bisa dilakukan secara concurrent tanpa blocking spider
        state.increment_page_count();
        state.mark_url_visited(response.url.to_string());
        
        // Custom parsing logic here
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    let crawler = CrawlerBuilder::new(MySpider).build().await?;
    crawler.start_crawl().await
}
```

**Note**: Perhatikan bahwa implementasi `Spider` sekarang menggunakan referensi immutable (`&self`) 
dan menerima parameter state terpisah (`state: &Self::State`). Ini memungkinkan concurrent crawling 
yang lebih efisien karena menghilangkan kebutuhan akan mutex pada spider itu sendiri.

**Important**: Make sure to import the prelude with `use spider_lib::prelude::*;` to bring the necessary items into scope for the macro to work properly.

## Contributing

We welcome contributions to the spider-lib project! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## Documentation

For more detailed documentation, visit [https://docs.rs/spider-lib](https://docs.rs/spider-lib)
