use crate::schema;
use crate::AppState;
use axum::extract;
use axum::response::IntoResponse;
use diesel::prelude::Queryable;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::Selectable;
use diesel::SelectableHelper;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct QuizInput {
	author: i32,
	title: String,
	description: String,
	questions: Vec<QuestionInput>,
}

#[derive(serde::Deserialize)]
pub struct QuestionInput {
	name: String,
	options: Vec<String>,
	answers: Vec<i16>,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::quiz)]
pub struct QuizResponse {
	id: i32,
	author: i32,
	title: String,
	description: String,
}

pub async fn create(
	extract::State(state): extract::State<Arc<AppState>>,
	extract::Json(data): extract::Json<QuizInput>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
	state
		.connection()
		.await?
		.transaction::<i32, diesel::result::Error, _>(|connection| {
			async move {
				let id = diesel::insert_into(schema::quiz::table)
					.values((
						schema::quiz::title.eq(data.title),
						schema::quiz::author.eq(data.author),
						schema::quiz::description.eq(data.description),
					))
					.returning(schema::quiz::id)
					.get_result::<i32>(connection)
					.await?;

				diesel::insert_into(schema::question::table)
					.values(
						data.questions
							.into_iter()
							.map(|q| {
								(
									schema::question::quiz.eq(id),
									schema::question::name.eq(q.name),
									schema::question::options.eq(q.options),
									schema::question::answers.eq(q.answers),
								)
							})
							.collect::<Vec<_>>(),
					)
					.execute(connection)
					.await?;

				Ok(id)
			}
			.scope_boxed()
		})
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(axum::http::StatusCode::CREATED)
}

pub async fn get_all(
	extract::State(state): extract::State<Arc<AppState>>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
	let result = schema::quiz::table
		.select(QuizResponse::as_select())
		.left_join(rhs)
		.get_results::<QuizResponse>(&mut state.connection().await?)
		.await;
}
