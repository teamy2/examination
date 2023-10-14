mod routes;

use axum::routing::{get, post};
use diesel::Connection;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let app = axum::Router::new().route("/", post(routes::auth::root));

	let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
	axum::Server::bind(&addr)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();

	let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL.");
	diesel::pg::PgConnection::establish(&database_url).unwrap();
}
