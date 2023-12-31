const url = 'https://api.healthhunt.xyz';

export type Quiz = {
	id?: number;
	author: number;
	title: string;
	description: string;
	questions?: Question[];
};

export type Question = {
	name: string;
	options: string[];
	answers: number[];
};

export type User = {
	username: string;
	password: string;
};

export type UserData = {
	id: number;
	username: string;
};

export async function getQuizzes(): Promise<Quiz[]> {
	const response = await get('/quizzes');
	const quizArray: Quiz[] = await response.json();
	return quizArray;
}

export async function getQuiz(id: number): Promise<Quiz> {
	const response = await get('/quizzes/' + id);
	const quiz: Quiz = await response.json();
	return quiz;
}

export async function getCreatedQuizzes(userId: number): Promise<Quiz[]> {
	const response = await get(`/quizzes/created?user=${userId}`);
	const quizArray: Quiz[] = await response.json();
	return quizArray;
}

export async function getCompletedQuizzes(userId: number): Promise<Quiz[]> {
	const response = await get(`/quizzes/completed?user=${userId}`);
	const quizArray: Quiz[] = await response.json();
	return quizArray;
}

export async function login(
	username: string,
	password: string
): Promise<UserData | undefined> {
	const response = await post<User>('/auth/login', {
		username: username,
		password: password,
	});

	if (response.status !== 200) return undefined;

	return response.json();
}

export async function register(
	username: string,
	password: string
): Promise<boolean> {
	const response = await post<User>('/auth/register', {
		username: username,
		password: password,
	});

	return response.status === 201;
}

export async function createQuiz(quiz: Quiz): Promise<number | undefined> {
	const response = await post<Quiz>('/quizzes/create', quiz);

	if (response.status !== 201) return undefined;

	const json = await response.json();

	return json.id;
}

export async function submitQuiz(quizId: number, userId: number, answers: Set<number>[]): Promise<{ results: boolean[][], correct_count: number } | undefined> {
	const response = await post(`/quizzes/${quizId}/complete?user=${userId}`, answers.map(a => [...a]));
	if (response.status !== 200) return undefined;

	return response.json();
}

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
