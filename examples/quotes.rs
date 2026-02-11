// Use the prelude for easy access to common types and traits.
use spider_lib::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;

#[scraped_item]
pub struct QuoteItem {
    pub text: String,
    pub author: String,
}

// State untuk tracking jumlah halaman yang telah diproses
#[derive(Clone, Default)]
pub struct QuotesSpiderState {
    page_count: Arc<AtomicUsize>,
    visited_urls: Arc<DashMap<String, bool>>,
}

impl QuotesSpiderState {
    pub fn increment_page_count(&self) {
        self.page_count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get_page_count(&self) -> usize {
        self.page_count.load(Ordering::SeqCst)
    }
    
    pub fn mark_url_visited(&self, url: String) {
        self.visited_urls.insert(url, true);
    }
}

pub struct QuotesSpider;

#[async_trait]
impl Spider for QuotesSpider {
    type Item = QuoteItem;
    type State = QuotesSpiderState;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://quotes.toscrape.com/"]
    }

    async fn parse(&self, response: Response, state: &Self::State) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Update state - bisa dilakukan secara concurrent tanpa blocking spider
        state.increment_page_count();
        state.mark_url_visited(response.url.to_string());
        
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("spider_lib=info,spider_core=info,spider_downloader=info,spider_middleware=info,spider_pipeline=info,spider_util=info"))
        .init();

    // The builder defaults to using ReqwestClientDownloader
    let crawler = CrawlerBuilder::new(QuotesSpider).build().await?;

    crawler.start_crawl().await?;

    Ok(())
}
