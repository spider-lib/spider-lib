use spider_lib::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[scraped_item]
    pub struct TestItem {
        pub title: String,
    }

    // State untuk testing
    #[derive(Clone, Default)]
    pub struct TestSpiderState {
        page_count: Arc<AtomicUsize>,
        visited_urls: Arc<DashMap<String, bool>>,
    }

    impl TestSpiderState {
        pub fn increment_page_count(&self) {
            self.page_count.fetch_add(1, Ordering::SeqCst);
        }
        
        pub fn mark_url_visited(&self, url: String) {
            self.visited_urls.insert(url, true);
        }
    }

    pub struct TestSpider;

    #[async_trait]
    impl Spider for TestSpider {
        type Item = TestItem;
        type State = TestSpiderState;

        fn start_urls(&self) -> Vec<&'static str> {
            // Using a simple static server for testing
            vec!["https://httpbin.org/html"]  // Simple HTML page for testing
        }

        async fn parse(&self, response: Response, state: &Self::State) -> Result<ParseOutput<Self::Item>, SpiderError> {
            // Update state
            state.increment_page_count();
            state.mark_url_visited(response.url.to_string());
            
            let html = response.to_html()?;
            let mut output = ParseOutput::new();

            // Extract title as a simple test
            if let Some(title_elem) = html.select(&"title".to_selector()?).next() {
                let title = title_elem.text().collect::<String>();
                output.add_item(TestItem { title });
            }

            Ok(output)
        }
    }

    #[tokio::test]
    async fn test_basic_crawling() {
        let crawler = CrawlerBuilder::new(TestSpider)
            .max_concurrent_downloads(2)  // Small number for testing
            .max_parser_workers(2)
            .channel_capacity(100)
            .build()
            .await
            .expect("Failed to build crawler");

        let stats = crawler.get_stats(); // Get stats before starting crawl
        let result = crawler.start_crawl().await;

        assert!(result.is_ok(), "Crawling should complete successfully");

        let requests_succeeded = stats.requests_succeeded.load(std::sync::atomic::Ordering::SeqCst);
        let items_scraped = stats.items_scraped.load(std::sync::atomic::Ordering::SeqCst);

        println!("Test completed: {} requests succeeded, {} items scraped",
                 requests_succeeded, items_scraped);

        // At minimum, we should have processed one request and potentially scraped an item
        assert!(requests_succeeded >= 1, "At least one request should succeed");
    }
}