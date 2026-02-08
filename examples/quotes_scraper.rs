// Use the prelude for easy access to common types and traits.
use spider_lib::prelude::*;

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
}

#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    // The builder defaults to using ReqwestClientDownloader
    let crawler = CrawlerBuilder::new(QuotesSpider).build().await?;

    crawler.start_crawl().await?;

    Ok(())
}
