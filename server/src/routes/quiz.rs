use std::collections::HashMap;

use axum::Json;
use axum::{extract, http::StatusCode};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};

use crate::models::{Question, Quiz, QuizExternal};
use crate::schema;
use crate::AppState;

pub async fn create(
	extract::State(state): extract::State<AppState>,
	extract::Json(mut data): extract::Json<QuizExternal>,
) -> crate::RouteResult<StatusCode> {
	data.strip_id();

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
	extract::State(state): extract::State<AppState>,
) -> crate::RouteResult<Json<Vec<QuizExternal>>> {
	let quizzes = schema::quiz::table
		.select(Quiz::as_select())
		.get_results::<Quiz>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

	let mut quizzes = quizzes
		.into_iter()
		.map(|q| (q.id, q.into()))
		.collect::<HashMap<_, QuizExternal>>();

	let questions = schema::question::table
		.select(Question::as_select())
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

	for question in questions {
		if let Some(quiz) = quizzes.get_mut(&question.quiz) {
			quiz.questions.push(question.into());
		}
	}

	Ok(Json(quizzes.into_values().collect()))
}

pub async fn get_one(
	extract::State(state): extract::State<AppState>,
	extract::Path(id): extract::Path<i32>,
) -> crate::RouteResult<Json<QuizExternal>> {
	let mut quiz: QuizExternal = schema::quiz::table
		.select(Quiz::as_select())
		.filter(schema::quiz::id.eq(id))
		.get_result::<Quiz>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
		.into();

	quiz.questions = schema::question::table
		.select(Question::as_select())
		.filter(schema::question::quiz.eq(id))
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
		.into_iter()
		.map(|q| q.into())
		.collect();

	Ok(Json(quiz))
}

#[derive(serde::Deserialize)]
pub struct AuthorInput {
	pub author: i32,
}

pub async fn get_created(
	extract::State(state): extract::State<AppState>,
	extract::Query(query): extract::Query<AuthorInput>,
) -> crate::RouteResult<Json<Vec<QuizExternal>>> {
	let quizzes = schema::quiz::table
		.select(Quiz::as_select())
		.filter(schema::quiz::author.eq(query.author))
		.get_results::<Quiz>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

	let mut quizzes = quizzes
		.into_iter()
		.map(|q| (q.id, q.into()))
		.collect::<HashMap<_, QuizExternal>>();

	let questions = schema::question::table
		.select(Question::as_select())
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

	for question in questions {
		if let Some(quiz) = quizzes.get_mut(&question.quiz) {
			quiz.questions.push(question.into());
		}
	}

	Ok(Json(quizzes.into_values().collect()))
}
