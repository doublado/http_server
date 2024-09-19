use hyper::{Body, Request, Response};
use crate::handlers::BoxedResponse;
use futures::FutureExt;

// Definerer funktionen 'subtract', der håndterer subtraktion af to tal fra forespørgslens query-parametre
pub fn subtract(req: Request<Body>) -> BoxedResponse {
    async move {
        // Ekstraherer query-parametre (f.eks. ?a=5&b=3)
        let query = req.uri().query().unwrap_or("");

        let mut a = 0.0;
        let mut b = 0.0;

        // Gennemløber query-parametrene og tildeler værdierne til 'a' og 'b'
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "a" => a = value.parse::<f64>().unwrap_or(0.0),
                "b" => b = value.parse::<f64>().unwrap_or(0.0),
                _ => (),
            }
        }

        // Beregner differencen mellem 'a' og 'b'
        let difference = a - b;
        let response_body = format!("Resultat: {}", difference);

        // Returnerer HTTP-responsen med resultatet
        Ok(Response::new(Body::from(response_body)))
    }
    .boxed()
}