<script lang="ts">
	import { page } from '$app/stores';
	import { getQuiz, submitQuiz, type Quiz } from '$lib/api';
	import { isLoggedIn, user } from '$lib/auth';

	import QuizAnswers from '$lib/components/QuizAnswers.svelte';

	import { onMount } from 'svelte';

	const id = parseInt($page.params.id);

	let quiz: Quiz;
	let curQuestion = 0;
	let answers: Set<number>[] = [];
	let correct: number | undefined;
	let results: boolean[][];

	let submitted = false;

	onMount(async () => {
		if (!(await isLoggedIn())) return goto('/login');

		quiz = await getQuiz(id);
	});

	async function handleSubmit() {
		submitted = true;

		const response = await submitQuiz(quiz.id!, $user?.id ?? 0, answers);

		correct = response?.correct_count ?? 0;
		results = response?.results ?? [];

		if (response) {
			curQuestion = 0;
		}
	}
</script>

{#if quiz}
	<div class="drawer lg:drawer-open">
		<input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col">
			<div class="content-center p-8 lg:p-12 grid gap-8">
				<span>
					<h1 class="top-8 text-5xl text-center">{quiz.title}</h1>
					<h1 class="text-center">
						by <span class="font-bold">{quiz.author}</span>
					</h1>
				</span>

				{#if correct !== undefined}
					You got {correct} / {quiz.questions?.length ?? 0} correct!
				{/if}

				<QuizAnswers
					{answers}
					{results}
					currentQuestion={curQuestion}
					question={quiz.questions[curQuestion].name}
					options={quiz.questions[curQuestion].options}
				/>

				<div class="flex gap-6">
					{#if curQuestion !== 0}
						<button
							class="btn btn-primary btn-lg"
							on:click={() => curQuestion--}
						>
							Back
						</button>
					{/if}

					{#if curQuestion !== quiz.questions.length - 1}
						<button
							class="btn btn-primary btn-lg"
							on:click={() => {
								if (curQuestion < quiz.questions.length - 1) curQuestion++;
							}}
						>
							Next
						</button>
					{/if}

					{#if !submitted && curQuestion === quiz.questions.length - 1}
						<button
							class="btn btn-primary btn-lg ml-auto"
							on:click={handleSubmit}
						>
							Submit
						</button>
					{/if}
				</div>
			</div>
			<label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden"
				>Open drawer</label
			>
		</div>
		<div class="drawer-side">
			<label
				for="my-drawer-2"
				aria-label="close sidebar"
				class="drawer-overlay"
			/>
			<ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
				{#each quiz.questions ?? [] as question, index}
					<li>
						<button on:click={() => (curQuestion = index)}
							>{index + 1}. {question.name}</button
						>
					</li>
				{/each}
			</ul>
		</div>
	</div>
{/if}
