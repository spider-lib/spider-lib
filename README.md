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
spider-lib = "0.5.1"
```

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
- `pipeline-streaming-json` - Enable streaming JSON functionality

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
use spider_lib::{Crawler, CrawlerBuilder, Spider, SpiderError};
use spider_lib::prelude::*;

#[derive(Default)]
struct MySpider;

#[spider_macro::scraped_item]
struct MyItem {
    title: String,
    url: String,
}

#[async_trait::async_trait]
impl Spider for MySpider {
    type Item = MyItem;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://example.com"]
    }

    async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
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

## Contributing

We welcome contributions to the spider-lib project! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Documentation

For more detailed documentation, visit [https://docs.rs/spider-lib](https://docs.rs/spider-lib)
