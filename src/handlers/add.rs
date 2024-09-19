use hyper::{Body, Request, Response};
use crate::handlers::BoxedResponse;
use futures::FutureExt;

// Definerer funktionen 'add', der håndterer addition af to tal fra forespørgslens query-parametre
pub fn add(req: Request<Body>) -> BoxedResponse {
    async move {
        // Ekstraher query-parametre (f.eks. ?a=5&b=3)
        let query = req.uri().query().unwrap_or("");

        let mut a = 0.0;
        let mut b = 0.0;

        // Parser query-parametrene og tildeler værdierne til 'a' og 'b'
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "a" => a = value.parse::<f64>().unwrap_or(0.0),
                "b" => b = value.parse::<f64>().unwrap_or(0.0),
                _ => (),
            }
        }

        // Beregner summen af 'a' og 'b'
        let sum = a + b;
        let response_body = format!("Resultat: {}", sum);

        // Returnerer HTTP-responsen med resultatet
        Ok(Response::new(Body::from(response_body)))
    }
    .boxed()
}