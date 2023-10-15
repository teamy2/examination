use axum::{extract, http::StatusCode, response::IntoResponse, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use sha2::Digest;

use crate::schema;
use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthInput {
	username: String,
	password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
	message: String,
	id: i32,
}

pub async fn login(
	extract::State(state): extract::State<AppState>,
	extract::Json(data): extract::Json<AuthInput>,
) -> crate::RouteResult<impl IntoResponse> {
	let hashed_password = hash(data.password);
	let result = schema::user::table
		.select(schema::user::id)
		.filter(schema::user::username.eq(data.username))
		.filter(schema::user::password.eq(hashed_password))
		.get_result::<i32>(&mut state.connection().await?)
		.await;

	Ok(match result {
		Ok(id) => Json(AuthResponse {
			message: "Success".to_string(),
			id,
		})
		.into_response(),
		Err(diesel::result::Error::NotFound) => {
			axum::http::StatusCode::UNAUTHORIZED.into_response()
		}
		Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
	})
}

pub async fn register(
	extract::State(state): extract::State<AppState>,
	extract::Json(data): extract::Json<AuthInput>,
) -> crate::RouteResult<StatusCode> {
	if data.username.len() > 16 {
		return Ok(axum::http::StatusCode::BAD_REQUEST);
	}

	let hashed_password = hash(data.password);
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

fn hash(data: impl AsRef<[u8]>) -> Vec<u8> {
	let mut hasher = sha2::Sha512::new();
	hasher.update(data.as_ref());
	hasher.finalize().to_vec()
}
