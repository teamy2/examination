mod routes;
mod schema;

use axum::http::StatusCode;
use axum::routing::{get, post};
use diesel::{Connection, PgConnection};
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::{net::SocketAddr, sync::Arc};

pub struct AppState {
	pub db: Pool<AsyncPgConnection>,
}

impl AppState {
	pub async fn connection(
		&self,
	) -> Result<Object<AsyncDieselConnectionManager<AsyncPgConnection>>, StatusCode> {
		self.db
			.get()
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
	}
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let state = AppState { db: get_pool() };

	let app = axum::Router::new()
		.route("/login", post(routes::auth::login))
		.route("/register", post(routes::auth::register))
		.with_state(Arc::new(state));

	let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
	axum::Server::bind(&addr)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
}

fn get_pool() -> Pool<AsyncPgConnection> {
	let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
		std::env::var("DATABASE_URL").expect("No DATABASE_URL."),
	);

	Pool::builder(manager).max_size(5).build().unwrap()
}
