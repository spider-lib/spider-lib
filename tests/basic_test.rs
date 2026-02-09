use spider_lib::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[scraped_item]
    pub struct TestItem {
        pub title: String,
    }

    pub struct TestSpider;

    #[async_trait]
    impl Spider for TestSpider {
        type Item = TestItem;

        fn start_urls(&self) -> Vec<&'static str> {
            // Using a simple static server for testing
            vec!["https://httpbin.org/html"]  // Simple HTML page for testing
        }

        async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
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

        let result = crawler.start_crawl().await;
        
        assert!(result.is_ok(), "Crawling should complete successfully");
        
        let stats = crawler.get_stats();
        let requests_succeeded = stats.requests_succeeded.load(std::sync::atomic::Ordering::SeqCst);
        let items_scraped = stats.items_scraped.load(std::sync::atomic::Ordering::SeqCst);
        
        println!("Test completed: {} requests succeeded, {} items scraped", 
                 requests_succeeded, items_scraped);
        
        // At minimum, we should have processed one request and potentially scraped an item
        assert!(requests_succeeded >= 1, "At least one request should succeed");
    }
}