extern crate rscrape;

use rscrape::Scraper;
use rscrape::fetchers::HttpClientFetcher;


fn main() {
    let f = HttpClientFetcher::new();
    let mut s = Scraper::<i32, _>::new(f);

    println!("Starting...");
    let res = s.scrape("https://news.ycombinator.com").unwrap();
    println!("Finished scraping");
}
