extern crate hyper;
extern crate kuchiki;

use std::error;
use std::io::Read;
use std::marker::PhantomData;

use hyper::method::Method;
use hyper::client::IntoUrl;
use kuchiki::Html;


pub mod fetchers;


/// The type parameter `P` is the type of a piece.
/// The type parameter `F` is the type of the fetcher.
pub struct Scraper<P, F>
    where F: fetchers::Fetcher
{
    fetcher: F,
    _piece: PhantomData<P>,
}

impl<P, F> Scraper<P, F>
where F: fetchers::Fetcher
{
    // TODO: builder?
    pub fn new(fetcher: F) -> Scraper<P, F> {
        Scraper {
            fetcher: fetcher,
            _piece: PhantomData::<P>,
        }
    }

    pub fn scrape<U: IntoUrl>(&mut self, start_url: U) -> Result<Vec<P>, Box<error::Error>> {
        let mut results = vec![];

        let mut next_url = Some(start_url);
        while let Some(url) = next_url {
            // TODO: page count

            // Fetch this URL.
            let mut r = try!(self.fetcher.fetch(Method::Get, url));

            // Read the page.
            let mut data = String::new();
            try!(r.read_to_string(&mut data));

            let doc = Html::from_string(data).parse();

            // TODO: divide, etc.


            // TODO:
            next_url = None;
        }

        Ok(results)
    }
}


#[cfg(test)]
mod tests {
    use kuchiki::Html;

    #[test]
    fn parsing_works() {
        let html = r"
    <!doctype html>
    <html>
        <head>
            <title>Foo</title>
        </head>
        <body>
            <p>Some stuff</p>
        </body>
    </html>";

        let doc = Html::from_string(html).parse();
        let ps = doc.select("p").unwrap().collect::<Vec<_>>();

        assert_eq!(ps.len(), 1);
    }
}
