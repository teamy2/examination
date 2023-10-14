mod routes;

use axum::routing::{get, post};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	let app = axum::Router::new().route("/", post(routes::auth::root));

	let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
	axum::Server::bind(&addr)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
}
