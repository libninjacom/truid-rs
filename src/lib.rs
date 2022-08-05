//! [`TruidClient`](struct.TruidClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;
use crate::model::*;

pub struct TruidClient {
    pub(crate) client: httpclient::Client,
}
impl TruidClient {}
impl TruidClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        TruidClient { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
}
