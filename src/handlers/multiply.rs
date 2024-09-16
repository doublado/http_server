use hyper::{Body, Request, Response};
use crate::handlers::BoxedResponse;
use futures::FutureExt;

pub fn multiply(req: Request<Body>) -> BoxedResponse {
  async move {
    let query = req.uri().query().unwrap_or("");

    let mut a = 0.0;
    let mut b = 0.0;

    for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
      match key.as_ref() {
        "a" => a = value.parse::<f64>().unwrap_or(0.0),
        "b" => b = value.parse::<f64>().unwrap_or(0.0),
        _ => (),
      }
    }

    let product = a * b;
    let response_body = format!("Result: {}", product);

    Ok(Response::new(Body::from(response_body)))
  }
  .boxed()
}