use crate::schema;
use crate::AppState;
use axum::{extract, response::IntoResponse, Json};
use diesel::ExpressionMethods;

use diesel_async::RunQueryDsl;
use sha2::Digest;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthInput {
	username: String,
	password: String,
}

pub async fn login(
	extract::State(state): extract::State<Arc<AppState>>,
	extract::Json(data): extract::Json<AuthInput>,
) -> impl IntoResponse {
	"sup"
}

pub async fn register(
	extract::State(state): extract::State<Arc<AppState>>,
	extract::Json(data): extract::Json<AuthInput>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
	let mut hasher = sha2::Sha512::new();
	hasher.update(data.username.as_bytes());
	let hashed_password = hasher.finalize();

	let result = diesel::insert_into(schema::user::table)
		.values((
			schema::user::username.eq(data.username),
			schema::user::password.eq(&hashed_password[..]),
		))
		.execute(&mut state.connection().await?)
		.await;

	Ok(if result.is_err() {
		axum::http::StatusCode::CONFLICT
	} else {
		axum::http::StatusCode::CREATED
	})
}
