// Use the prelude for easy access to common types and traits.
use spider_lib::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;

#[scraped_item]
pub struct BookItem {
    pub title: String,
    pub price: String,
    pub rating: String,
    pub availability: String,
    pub upc: String,
    pub tax: String,
    pub reviews: String,
    pub stock: String,
}

// State untuk tracking jumlah halaman dan buku yang telah diproses
#[derive(Clone, Default)]
pub struct BooksSpiderState {
    page_count: Arc<AtomicUsize>,
    book_count: Arc<AtomicUsize>,
    visited_urls: Arc<DashMap<String, bool>>,
}

impl BooksSpiderState {
    pub fn increment_page_count(&self) {
        self.page_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_book_count(&self) {
        self.book_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_page_count(&self) -> usize {
        self.page_count.load(Ordering::SeqCst)
    }

    pub fn get_book_count(&self) -> usize {
        self.book_count.load(Ordering::SeqCst)
    }

    pub fn mark_url_visited(&self, url: String) {
        self.visited_urls.insert(url, true);
    }
}

pub struct BooksSpider;

#[async_trait]
impl Spider for BooksSpider {
    type Item = BookItem;
    type State = BooksSpiderState;

    fn start_urls(&self) -> Vec<&'static str> {
        vec!["https://books.toscrape.com/"]
    }

    async fn parse(&self, response: Response, state: &Self::State) -> Result<ParseOutput<Self::Item>, SpiderError> {
        // Update state - bisa dilakukan secara concurrent tanpa blocking spider
        state.increment_page_count();
        state.mark_url_visited(response.url.to_string());

        let html = response.to_html()?;
        let mut output = ParseOutput::new();

        // Check if this is a category/listing page or a book detail page
        if html.select(&".product_main".to_selector()?).next().is_some() {
            // This is a book detail page
            let title = html
                .select(&".product_main h1".to_selector()?)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            let price = html
                .select(&".price_color".to_selector()?)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            // Extract rating from star rating class
            let rating = html
                .select(&".star-rating".to_selector()?)
                .next()
                .and_then(|e| e.attr("class"))
                .map(|class| {
                    class.split_whitespace()
                        .find(|&c| c != "star-rating")
                        .unwrap_or_default()
                        .to_string()
                })
                .unwrap_or_default();

            // Extract additional details from the table
            let mut upc = String::new();
            let mut tax = String::new();
            let mut reviews = String::new();
            let mut availability = String::new();

            for row in html.select(&".table.table-striped tr".to_selector()?) {
                if let (Some(label_elem), Some(value_elem)) = (
                    row.select(&"th".to_selector()?).next(),
                    row.select(&"td".to_selector()?).next()
                ) {
                    let label = label_elem.text().collect::<String>().trim().to_lowercase();
                    let value = value_elem.text().collect::<String>().trim().to_string();

                    match label.as_str() {
                        "upc" => upc = value,
                        "tax" => tax = value,
                        "number of reviews" => reviews = value,
                        "availability" => availability = value,
                        _ => {}
                    }
                }
            }

            output.add_item(BookItem {
                title,
                price,
                rating,
                availability,
                upc,
                tax,
                reviews,
                stock: String::new(), // Initialize stock field
            });

            state.increment_book_count();
        } else {
            // This is a category/listing page
            for book in html.select(&"article.product_pod".to_selector()?) {
                // Extract title
                let _title = book
                    .select(&"h3 a".to_selector()?)
                    .next()
                    .and_then(|a| a.attr("title"))
                    .unwrap_or_default()
                    .to_string();

                // Extract price
                let _price = book
                    .select(&".price_color".to_selector()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                // Extract rating
                let rating_class = book
                    .select(&".star-rating".to_selector()?)
                    .next()
                    .and_then(|e| e.attr("class"))
                    .unwrap_or_default();
                
                let _rating = rating_class
                    .split_whitespace()
                    .find(|&c| c != "star-rating")
                    .unwrap_or_default()
                    .to_string();

                // Follow link to individual book page to get more details
                if let Some(book_link) = book
                    .select(&"h3 a".to_selector()?)
                    .next()
                    .and_then(|a| a.attr("href"))
                {
                    let book_url = response.url.join(book_link)?;
                    
                    // Create a request to the book detail page
                    output.add_request(Request::new(book_url));
                }

                state.increment_book_count();
            }

            // Handle pagination - find next page link
            if let Some(next_href) = html
                .select(&".next > a[href]".to_selector()?)
                .next()
                .and_then(|a| a.attr("href"))
            {
                let next_url = response.url.join(next_href)?;
                output.add_request(Request::new(next_url));
            }
        }

        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<(), SpiderError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("spider_lib=info,spider_core=info,spider_downloader=info,spider_middleware=info,spider_pipeline=info,spider_util=info"))
        .init();

    // The builder defaults to using ReqwestClientDownloader
    let crawler = CrawlerBuilder::new(BooksSpider).build().await?;

    crawler.start_crawl().await?;

    Ok(())
}
