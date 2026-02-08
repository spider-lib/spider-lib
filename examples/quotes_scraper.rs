use spider_lib::{ScrapedItem, SpiderError};
use spider_macro::scraped_item;

#[scraped_item]
struct QuoteItem {
    text: String,
    author: String,
    author_link: String,
}


fn main() -> Result<(), SpiderError> {
    // Initialize tracing with the specified configuration
    tracing_subscriber::fmt()
        .with_env_filter("info,spider_lib=debug")
        .init();
    
    Ok(())
}
