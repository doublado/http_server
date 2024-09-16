pub mod add;
pub mod subtract;
pub mod multiply;
pub mod divide;

pub use add::add;
pub use subtract::subtract;
pub use multiply::multiply;
pub use divide::divide;

use hyper::{Body, Response};
use futures::future::BoxFuture;

pub type BoxedResponse = BoxFuture<'static, Result<Response<Body>, hyper::Error>>;