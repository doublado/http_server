mod handlers;
mod models;
mod router;

use crate::handlers::{add, multiply, subtract, divide};
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
    // Indlæser ruter fra konfigurationsfilen
    let routes = load_routes("routes.json");

    // Initialiserer funktionsregistret
    let mut function_registry: HashMap<String, router::Handler> = HashMap::new();

    // Registrerer 'add' funktionen
    function_registry.insert(
        "add".to_string(),
        Arc::new(|req| add(req)),
    );

    // Registrerer 'subtract' funktionen
    function_registry.insert(
        "subtract".to_string(),
        Arc::new(|req| subtract(req)),
    );

    // Registrerer 'multiply' funktionen
    function_registry.insert(
        "multiply".to_string(),
        Arc::new(|req| multiply(req)),
    );

    // Registrerer 'divide' funktionen
    function_registry.insert(
        "divide".to_string(),
        Arc::new(|req| divide(req)),
    );

    // Initialiserer routeren
    let mut router = Router::new();

    // Tilføjer ruter til routeren baseret på konfigurationen
    for route in routes {
        if let Some(handler) = function_registry.get(&route.function) {
            router.add_route(route.path.clone(), handler.clone());
            println!("Tilføjede rute: {} -> {}", route.path, route.function);
        } else {
            eprintln!(
                "Handler funktion '{}' ikke fundet for sti '{}'",
                route.function, route.path
            );
        }
    }

    // Gør routeren trådsikker ved at pakke den i en Arc
    let router = Arc::new(router);

    // Sætter adressen, serveren skal lytte på
    let addr = ([127, 0, 0, 1], 3000).into();

    // Opretter en service, der håndterer indgående forespørgsler
    let make_svc = make_service_fn(move |_conn| {
        let router = router.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let router = router.clone();
                async move { router.route(req).await }
            }))
        }
    });

    // Opretter og starter serveren
    let server = Server::bind(&addr).serve(make_svc);

    println!("Lytter på http://{}", addr);

    // Kører serveren
    if let Err(e) = server.await {
        eprintln!("Server fejl: {}", e);
    }
}

// Funktion til at indlæse ruter fra konfigurationsfilen
fn load_routes(file_path: &str) -> Vec<Route> {
    let file = File::open(file_path).expect("Kan ikke åbne rute-filen");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Fejl ved parsing af rute-filen")
}