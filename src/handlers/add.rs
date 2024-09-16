use hyper::{Body, Request, Response};
use crate::handlers::BoxedResponse;
use futures::FutureExt;

pub fn add(req: Request<Body>) -> BoxedResponse {
  async move {
    // Extract query parameters (e.g., ?a=5&b=3)
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

    let sum = a + b;
    let response_body = format!("Result: {}", sum);

    Ok(Response::new(Body::from(response_body)))
  }
  .boxed()
}