//! Streaming response example for spider-lib.
//!
//! This example demonstrates how to use the streaming response feature
//! to process large responses without loading the entire body into memory.

use spider_lib::prelude::*;

#[scraped_item]
pub struct LinkItem {
    pub url: String,
}

pub struct StreamingSpider;

#[async_trait]
impl Spider for StreamingSpider {
    type Item = LinkItem;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://httpbin.org/html"] // This returns HTML content
    }

    async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Traditional parsing - loads entire response body into memory
        println!("Parsing traditional response from: {}", response.url);

        // Extract links from the response
        let links = response.links();
        let mut output = ParseOutput::new();

        for link in links {
            println!("Found link: {} ({:?})", link.url, link.link_type);
            output.add_item(LinkItem {
                url: link.url.to_string(),
            });
        }

        Ok(output)
    }

    #[cfg(feature = "streaming")]
    async fn parse_streaming(
        &mut self,
        response: StreamingResponse,
    ) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Streaming parsing - processes response without loading entire body into memory
        println!("Parsing streaming response from: {}", response.url);

        // Extract links by consuming the streaming response
        let links = response
            .into_links()
            .await
            .map_err(|e| SpiderError::IoError(e.to_string()))?;
        let mut output = ParseOutput::new();

        for link in links {
            println!(
                "Found link (streaming): {} ({:?})",
                link.url, link.link_type
            );
            output.add_item(LinkItem {
                url: link.url.to_string(),
            });
        }

        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("Starting spider-lib with streaming response example...");

    // Build crawler with streaming feature enabled (when compiled with --features streaming)
    let crawler = CrawlerBuilder::new(StreamingSpider)
        .max_concurrent_downloads(2)
        .max_parser_workers(1)
        .build()
        .await?;

    crawler.start_crawl().await?;

    println!("Streaming example completed successfully!");
    Ok(())
}

