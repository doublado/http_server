use std::collections::HashMap;
use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::handlers::BoxedResponse;

pub type Handler = Arc<dyn Fn(Request<Body>) -> BoxedResponse + Send + Sync>;

pub struct Router {
  pub routes: HashMap<String, Handler>,
}

impl Router {
  pub fn new() -> Self {
    Router {
      routes: HashMap::new(),
    }
  }

  pub fn add_route(&mut self, path: String, handler: Handler) {
    self.routes.insert(path, handler);
  }

  pub async fn route(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();

    if let Some(handler) = self.routes.get(&path) {
      handler(req).await
    } else {
      Ok(Response::builder()
        .status(404)
        .body(Body::from("Not Found"))
        .unwrap())
    }
  }
}