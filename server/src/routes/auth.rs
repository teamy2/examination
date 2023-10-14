use axum::{extract, response::IntoResponse, Json};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Data {
	name: String,
	number: u8,
}

pub async fn root(extract::Json(data): extract::Json<Data>) -> impl IntoResponse {
	Json(Data {
		name: data.name,
		number: data.number,
	})
}
