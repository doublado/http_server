use hyper::{Body, Request, Response, StatusCode};
use crate::handlers::BoxedResponse;
use futures::FutureExt;
use serde_json::json;
use hyper::header::{CONTENT_TYPE, HeaderValue};

// Definerer funktionen 'divide', der håndterer division af to tal fra forespørgslens query-parametre
pub fn divide(req: Request<Body>) -> BoxedResponse {
    async move {
        // Ekstraher query-parametre (f.eks. ?a=5&b=3)
        let query = req.uri().query().unwrap_or("");

        let mut a_str = None;
        let mut b_str = None;

        // Gennemløber query-parametrene og tildeler værdierne til 'a_str' og 'b_str'
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "a" => a_str = Some(value.to_string()),
                "b" => b_str = Some(value.to_string()),
                _ => (),
            }
        }

        // Tjekker om både 'a' og 'b' er angivet
        if a_str.is_none() || b_str.is_none() {
            let error_response = json!({
                "error": "Manglende query-parametre 'a' og 'b'."
            })
            .to_string();

            let mut response = Response::new(Body::from(error_response));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );

            return Ok(response);
        }

        // Forsøger at parse 'a' og 'b' til f64
        let a: f64 = match a_str.unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                let error_response = json!({
                    "error": "Ugyldig værdi for 'a'. Skal være et tal."
                })
                .to_string();

                let mut response = Response::new(Body::from(error_response));
                *response.status_mut() = StatusCode::BAD_REQUEST;
                response.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );

                return Ok(response);
            }
        };

        let b: f64 = match b_str.unwrap().parse() {
            Ok(num) => num,
            Err(_) => {
                let error_response = json!({
                    "error": "Ugyldig værdi for 'b'. Skal være et tal."
                })
                .to_string();

                let mut response = Response::new(Body::from(error_response));
                *response.status_mut() = StatusCode::BAD_REQUEST;
                response.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );

                return Ok(response);
            }
        };

        // Tjekker for division med nul
        if b == 0.0 {
            let error_response = json!({
                "error": "Division med nul er ikke tilladt."
            })
            .to_string();

            let mut response = Response::new(Body::from(error_response));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );

            return Ok(response);
        }

        // Beregner kvotienten af 'a' og 'b'
        let quotient = a / b;

        // Opretter et JSON-objekt med resultatet
        let response_body = json!({ "result": quotient }).to_string();

        // Opretter HTTP-responsen med JSON-indhold og sætter 'Content-Type' headeren
        let mut response = Response::new(Body::from(response_body));
        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        // Returnerer HTTP-responsen med resultatet
        Ok(response)
    }
    .boxed()
}