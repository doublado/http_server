mod handlers;
mod models;
mod router;

use crate::handlers::{add, multiply, subtract}; // Keep the imports
use crate::models::Route;
use crate::router::Router;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  // Load routes from configuration file
  let routes = load_routes("routes.json");

  // Initialize the function registry
  let mut function_registry: HashMap<String, router::Handler> = HashMap::new();

  function_registry.insert(
    "add".to_string(),
    Arc::new(|req| add(req)), // Use the imported function directly
  );

  function_registry.insert(
    "subtract".to_string(),
    Arc::new(|req| subtract(req)), // Use the imported function directly
  );

  function_registry.insert(
    "multiply".to_string(),
    Arc::new(|req| multiply(req)), // Use the imported function directly
  );

  // Initialize the router
  let mut router = Router::new();

  for route in routes {
    if let Some(handler) = function_registry.get(&route.function) {
      router.add_route(route.path.clone(), handler.clone());
      println!("Added route: {} -> {}", route.path, route.function);
    } else {
      eprintln!(
        "Handler function '{}' not found for path '{}'",
        route.function, route.path
      );
    }
  }

  let router = Arc::new(router);

  // Set the address to serve on
  let addr = ([127, 0, 0, 1], 3000).into();

  // Create a service that handles incoming requests
  let make_svc = make_service_fn(move |_conn| {
    let router = router.clone();
    async move {
      Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
        let router = router.clone();
        async move { router.route(req).await }
      }))
    }
  });

  // Create the server
  let server = Server::bind(&addr).serve(make_svc);

  println!("Listening on http://{}", addr);

  // Run the server
  if let Err(e) = server.await {
    eprintln!("Server error: {}", e);
  }
}

// Function to load routes from the configuration file
fn load_routes(file_path: &str) -> Vec<Route> {
  let file = File::open(file_path).expect("Cannot open routes file");
  let reader = BufReader::new(file);
  serde_json::from_reader(reader).expect("Error parsing routes file")
}