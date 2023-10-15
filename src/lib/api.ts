const url = 'http://localhost:8000';

export type Quiz = {
	id?: number;
	author: string;
	title: string;
	description: string;
	questions?: Question[];
};

export type Question = {
	name: string;
	options: string[];
	answers?: number[];
};

export type User = {
	username: string;
	password: string;
};

export async function getQuizzes(): Promise<Quiz[]> {
	const response = await get('/quizzes/');
	const quizArray: Quiz[] = await response.json();
	return quizArray;
}

export async function getQuiz(id: number): Promise<Quiz> {
	const response = await get('/quizzes' + id);
	const quiz: Quiz = await response.json();
	return quiz;
}

export async function getCreatedQuizzes(): Promise<Quiz[]> {
	const response = await get('/quizzes/created');
	const quizArray: Quiz[] = await response.json();
	return quizArray;
}

export async function login(
	username: string,
	password: string
): Promise<boolean> {
	const response = await post<User>('/auth/login', {
		username: username,
		password: password,
	});

	return response.status === 200;
}

export async function register(
	username: string,
	password: string
): Promise<boolean> {
	const response = await post<User>('/auth/register', {
		username: username,
		password: password,
	});
	return response.status === 200;
}

export async function createQuiz(quiz: Quiz): Promise<boolean> {
	const response = await post<Quiz>('/quizzes/create', quiz);
	return response.status === 201;
}

export async function submitQuiz() {}

function post<T>(path: string, body: T): Promise<Response> {
	return fetch(url + path, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(body),
	});
}

function get(path: string): Promise<Response> {
	return fetch(url + path, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	});
}