use axum::{Router, routing::get};
use http::Request;
use hyper::Body;
use shared::{Boom, Pah};
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestIdLayer, RequestId};
use tracing::{info_span, Level};

mod middleware;

extern crate handlers;
#[macro_use]
extern crate timber;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().with_max_level(Level::INFO).init();

  let boom = Boom("Boom!".into());
  // let pah = Pah("Pah!".into());

  let app = Router::new()
  .route("/", get(handlers::greet))
  .route("/boom", get(handlers::boom))
  // .route("/pah", get(handlers::pah)) // will throw a type error
    .layer(axum::middleware::from_fn(middleware::log_request))
    .layer(
      // Let's create a tracing span for each request
      TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        // We get the request id from the extensions
        let request_id = request
          .extensions()
          .get::<RequestId>()
          .map(ToString::to_string)
          .unwrap_or_else(|| "unknown".into());
        // And then we put it along with other information into the `request` span
        info_span!(
            "request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
      }),
    )
    // This layer creates a new id for each request and puts it into the request extensions.
    // Note that it should be added after the Trace layer.
    .layer(RequestIdLayer)
    .with_state(boom);
    // .with_state(pah); // doesn't work

  axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
