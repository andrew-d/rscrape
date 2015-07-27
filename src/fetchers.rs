use std::error;
use std::io::Read;

use hyper::method::Method;
use hyper::client::{Client, IntoUrl};


/// Fetcher is the interface for things that can fetch a remote URL.
pub trait Fetcher {
    /// Called once at the beginning of the scrape.
    fn prepare(&mut self) -> Result<(), Box<error::Error>> {
        Ok(())
    }

    /// Called to retrieve each document from the remote server.
    fn fetch<U: IntoUrl>(&mut self, method: Method, url: U) -> Result<Box<Read>, Box<error::Error>>;

    /// Called once when the scrape is finished.  Can be used to clean up
    /// allocated resources or perform other cleanup actions.
    fn close(&mut self) {
    }
}


pub struct HttpClientFetcher {
    // TODO: prepare client function?
    // TODO: prepare request function?
    // TODO: process response function?
    c: Client,
}

impl HttpClientFetcher {
    pub fn new() -> Self {
        HttpClientFetcher {
            c: Client::new(),
        }
    }

    pub fn with_client(c: Client) -> Self {
        HttpClientFetcher {
            c: c,
        }
    }
}

impl Fetcher for HttpClientFetcher {
    fn fetch<U: IntoUrl>(&mut self, method: Method, url: U) -> Result<Box<Read>, Box<error::Error>> {
        // TODO: filters?
        let resp = try!(self.c.request(method, url).send());

        // TODO: post filters?

        Ok(Box::new(resp) as Box<Read>)
    }
}


#[cfg(test)]
mod tests {
    use hyper::method::Method;

    use super::{Fetcher, HttpClientFetcher};


    #[test]
    fn test_will_fetch() {
        let mut cf = HttpClientFetcher::new();
        let mut r = cf.fetch(Method::Get, "https://www.google.com").unwrap();

        let mut data = String::new();
        r.read_to_string(&mut data).unwrap();

        // Simple sanity check.
        assert!(data.len() > 10);
    }
}
