use super::schema;
use diesel::{Queryable, Selectable};

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::quiz)]
pub struct Quiz {
	pub id: i32,
	pub author: i32,
	pub title: String,
	pub description: String,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::question)]
pub struct Question {
	pub id: i32,
	pub quiz: i32,
	pub name: String,
	pub options: Vec<Option<String>>,
	pub answers: Vec<Option<i16>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct QuizExternal {
	pub id: Option<i32>,
	pub author: i32,
	pub title: String,
	pub description: String,
	pub questions: Vec<QuestionExternal>,
}

impl QuizExternal {
	pub fn strip_id(&mut self) {
		self.id = None;
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct QuestionExternal {
	pub name: String,
	pub options: Vec<String>,
	pub answers: Vec<i16>,
}

impl From<Question> for QuestionExternal {
	fn from(question: Question) -> Self {
		Self {
			name: question.name,
			options: question.options.into_iter().flatten().collect(),
			answers: question.answers.into_iter().flatten().collect(),
		}
	}
}

impl From<Quiz> for QuizExternal {
	fn from(quiz: Quiz) -> Self {
		Self {
			id: Some(quiz.id),
			author: quiz.author,
			title: quiz.title,
			description: quiz.description,
			questions: Vec::new(),
		}
	}
}
