use serde::Deserialize;

#[derive(Deserialize)]
pub struct Route {
  pub path: String,
  pub function: String,
}