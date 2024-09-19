use std::collections::HashMap;
use hyper::{Body, Request, Response};
use std::sync::Arc;

use crate::handlers::BoxedResponse;

// Definerer typen 'Handler' som en trådsikker og synkroniseret funktion, der tager en Request<Body> og returnerer en BoxedResponse
pub type Handler = Arc<dyn Fn(Request<Body>) -> BoxedResponse + Send + Sync>;

// Definerer strukturen 'Router', som indeholder en samling af ruter
pub struct Router {
    pub routes: HashMap<String, Handler>,
}

impl Router {
    // Opretter en ny instans af Router
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    // Tilføjer en ny rute til routeren
    pub fn add_route(&mut self, path: String, handler: Handler) {
        self.routes.insert(path, handler);
    }

    // Finder og kalder den passende handler baseret på anmodningens sti
    pub async fn route(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let path = req.uri().path().to_string();

        if let Some(handler) = self.routes.get(&path) {
            handler(req).await
        } else {
            Ok(Response::builder()
                .status(404)
                .body(Body::from("Ikke fundet"))
                .unwrap())
        }
    }
}