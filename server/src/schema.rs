// @generated automatically by Diesel CLI.

diesel::table! {
	question (id) {
		id -> Int4,
		quiz -> Int4,
		name -> Text,
		options -> Array<Nullable<Text>>,
		answers -> Array<Nullable<Int2>>,
	}
}

diesel::table! {
	quiz (id) {
		id -> Int4,
		author -> Int4,
		title -> Text,
		description -> Text,
	}
}

diesel::table! {
	user (id) {
		id -> Int4,
		#[max_length = 16]
		username -> Varchar,
		password -> Bytea,
	}
}

diesel::allow_tables_to_appear_in_same_query!(question, quiz, user,);
