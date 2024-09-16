pub mod add;
pub mod subtract;
pub mod multiply;

pub use add::add;
pub use subtract::subtract;
pub use multiply::multiply;

use hyper::{Body, Response};
use futures::future::BoxFuture;

pub type BoxedResponse = BoxFuture<'static, Result<Response<Body>, hyper::Error>>;