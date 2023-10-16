use std::collections::HashMap;

use axum::{
	extract::{Json, Path, Query, State},
	http::StatusCode,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};

use crate::models::{Question, Quiz, QuizExternal};
use crate::schema;
use crate::AppState;

#[derive(serde::Serialize)]
pub struct CreateOutput {
	pub id: i32,
}

pub async fn create(
	State(state): State<AppState>,
	Json(mut data): Json<QuizExternal>,
) -> crate::RouteResult<(StatusCode, Json<CreateOutput>)> {
	data.strip_id();

	let id = state
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
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::CREATED, Json(CreateOutput { id })))
}

pub async fn get_all(State(state): State<AppState>) -> crate::RouteResult<Json<Vec<QuizExternal>>> {
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
	State(state): State<AppState>,
	Path(id): Path<i32>,
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
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.into_iter()
		.map(|q| q.into())
		.collect();

	Ok(Json(quiz))
}

#[derive(serde::Deserialize)]
pub struct UserInput {
	pub user: i32,
}

pub async fn get_created(
	State(state): State<AppState>,
	Query(input): Query<UserInput>,
) -> crate::RouteResult<Json<Vec<QuizExternal>>> {
	let quizzes = schema::quiz::table
		.select(Quiz::as_select())
		.filter(schema::quiz::author.eq(input.user))
		.get_results::<Quiz>(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let mut quizzes = quizzes
		.into_iter()
		.map(|q| (q.id, q.into()))
		.collect::<HashMap<_, QuizExternal>>();

	let questions = schema::question::table
		.select(Question::as_select())
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	for question in questions {
		if let Some(quiz) = quizzes.get_mut(&question.quiz) {
			quiz.questions.push(question.into());
		}
	}

	Ok(Json(quizzes.into_values().collect()))
}

pub async fn get_completed(
	State(state): State<AppState>,
	Query(input): Query<UserInput>,
) -> crate::RouteResult<Json<Vec<QuizExternal>>> {
	let completed = schema::completion::table
		.select(schema::completion::quiz)
		.filter(schema::completion::user.eq(input.user))
		.into_boxed();

	let quizzes = schema::quiz::table
		.select(Quiz::as_select())
		.filter(schema::quiz::author.eq(input.user))
		.filter(schema::quiz::id.eq_any(completed))
		.get_results::<Quiz>(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let mut quizzes = quizzes
		.into_iter()
		.map(|q| (q.id, q.into()))
		.collect::<HashMap<_, QuizExternal>>();

	let questions = schema::question::table
		.select(Question::as_select())
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	for question in questions {
		if let Some(quiz) = quizzes.get_mut(&question.quiz) {
			quiz.questions.push(question.into());
		}
	}

	Ok(Json(quizzes.into_values().collect()))
}

#[derive(serde::Serialize)]
pub struct CompleteOutput {
	pub results: Vec<Vec<bool>>,
	pub correct_count: usize,
}

pub async fn complete(
	State(state): State<AppState>,
	Path(id): Path<i32>,
	Query(input): Query<UserInput>,
	Json(answers): Json<Vec<Vec<i16>>>,
) -> crate::RouteResult<Json<CompleteOutput>> {
	let questions: Vec<Question> = schema::question::table
		.select(Question::as_select())
		.filter(schema::question::quiz.eq(id))
		.order(schema::question::id.asc())
		.get_results::<Question>(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	// construct a 2d array of booleans, where each row is a question and each column is
	// whether the user correctly selected (or didnt select) the option
	let mut results = Vec::with_capacity(questions.len());
	let mut correct_count = 0;

	for (question, answer) in questions.into_iter().zip(answers.into_iter()) {
		let mut question_correct = Vec::with_capacity(answer.len());
		let mut all_correct = true;

		for i in 0..question.options.len() {
			let is_answer = question.answers.contains(&Some(i as i16));
			let is_selected = answer.contains(&(i as i16));

			question_correct.push(is_answer == is_selected);

			if is_answer != is_selected {
				all_correct = false;
			}
		}

		results.push(question_correct);

		if all_correct {
			correct_count += 1;
		}
	}

	diesel::insert_into(schema::completion::table)
		.values((
			schema::completion::quiz.eq(id),
			schema::completion::user.eq(input.user),
			schema::completion::score.eq(correct_count as i16),
		))
		.on_conflict((schema::completion::quiz, schema::completion::user))
		.do_update()
		.set(schema::completion::score.eq(correct_count as i16))
		.execute(&mut state.connection().await?)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(Json(CompleteOutput {
		results,
		correct_count,
	}))
}
