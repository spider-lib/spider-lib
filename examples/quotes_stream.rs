//! Stream quotes scraper example for spider-lib.
//!
//! This example demonstrates how to use the stream response feature
//! to scrape quotes from http://quotes.toscrape.com/ without loading the entire body into memory.

use spider_lib::prelude::*;

#[scraped_item]
pub struct QuoteItem {
    pub text: String,
    pub author: String,
}

pub struct StreamQuotesSpider;

#[async_trait]
impl Spider for StreamQuotesSpider {
    type Item = QuoteItem;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://quotes.toscrape.com/"]
    }

    async fn parse(&mut self, response: Response) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Traditional parsing - loads entire response body into memory
        println!("Parsing traditional response from: {}", response.url);

        let html = response.to_html()?;
        let mut output = ParseOutput::new();

        for quote in html.select(&".quote".to_selector()?) {
            let text = quote
                .select(&".text".to_selector()?)
                .next()
                .map(|e| e.text().collect())
                .unwrap_or_default();
            let author = quote
                .select(&".author".to_selector()?)
                .next()
                .map(|e| e.text().collect())
                .unwrap_or_default();
            output.add_item(QuoteItem { text, author });
        }

        if let Some(next_href) = html
            .select(&".next > a[href]".to_selector()?)
            .next()
            .and_then(|a| a.attr("href"))
        {
            let next_url = response.url.join(next_href)?;
            output.add_request(Request::new(next_url));
        }

        Ok(output)
    }

    async fn parse_stream(
        &mut self,
        response: StreamResponse,
    ) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Stream parsing - processes response without loading entire body into memory
        println!("Parsing stream response from: {}", response.url);

        // Extract links by consuming the stream response
        let links = response
            .into_links()
            .await
            .map_err(|e| SpiderError::IoError(e.to_string()))?;
        let mut output = ParseOutput::new();

        // Look for pagination links in the stream response
        for link in links {
            if link.url.as_str().contains("/page/") {
                println!("Found pagination link: {}", link.url);
                output.add_request(Request::new(link.url.clone()));
            } else {
                println!("Found link: {} ({:?})", link.url, link.link_type);
            }
        }

        Ok(output)
    }
}


#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    // The builder defaults to using ReqwestClientDownloader
    let crawler = CrawlerBuilder::new(StreamQuotesSpider).build().await?;

    let stats = crawler.get_stats();
    crawler.start_crawl().await?;
    println!("{}", stats);

    Ok(())
}
