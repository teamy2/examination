#![feature(type_alias_impl_trait)]

mod models;
mod routes;
mod schema;

use std::{net::SocketAddr, sync::Arc};

use axum::{
	http::StatusCode,
	routing::{get, post},
};
use diesel_async::{
	pooled_connection::{
		deadpool::{Object, Pool},
		AsyncDieselConnectionManager,
	},
	AsyncPgConnection,
};
use tower::ServiceBuilder;
use tower_http::cors::Any;

pub type RouteResult<T> = Result<T, axum::http::StatusCode>;
pub type AppState = Arc<State>;

pub struct State {
	pub db: Pool<AsyncPgConnection>,
}

impl State {
	pub async fn connection(&self) -> Result<Object<AsyncPgConnection>, StatusCode> {
		self.db
			.get()
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
	}
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let state = State { db: get_pool() };

	let app = axum::Router::new()
		.route("/auth/login", post(routes::auth::login))
		.route("/auth/register", post(routes::auth::register))
		.route("/quizzes/create", post(routes::quiz::create))
		.route("/quizzes/:id", get(routes::quiz::get_one))
		.route("/quizzes/:id/complete", post(routes::quiz::complete))
		.route("/quizzes/created", get(routes::quiz::get_created))
		.route("/quizzes/completed", get(routes::quiz::get_completed))
		.route("/quizzes", get(routes::quiz::get_all))
		.layer(
			ServiceBuilder::new().layer(
				tower_http::cors::CorsLayer::new()
					.allow_origin(Any)
					.allow_headers(Any)
					.allow_methods(Any),
			),
		)
		.with_state(Arc::new(state));

	let addr = SocketAddr::from(([0, 0, 0, 0], 8001));

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
