<script lang="ts">
	import { page } from '$app/stores';
	import { getQuiz, submitQuiz, type Quiz } from '$lib/api';
	import { user } from '$lib/auth';

	import QuizAnswers from '$lib/components/QuizAnswers.svelte';

	import { onMount } from 'svelte';

	const id = parseInt($page.params.id);

	let quiz: Quiz;
	let curQuestion = 1;
	let answers: Set<number>[] = [];
	let correct: number | undefined;
	let results: boolean[][];

	onMount(async () => {
		quiz = await getQuiz(id);
	});

	async function handleSubmit() {
		const response = await submitQuiz(quiz.id!, $user?.id ?? 0, answers);

		correct = response?.correct_count ?? 0;
		results = response?.results ?? [];

		if (response) {
			answers = [];
			curQuestion = 1;
			quiz = await getQuiz(id);
		}
	}
</script>

{#if quiz}
	<div class="drawer lg:drawer-open">
		<input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col">
			<div class="h-20 content-center">
				<h1 class="top-8 text-3xl">{quiz.title}</h1>
				<h1>
					By: {quiz.author}
				</h1>

				{#if correct !== undefined}
					You got {correct} / {quiz.questions?.length ?? 0} correct!
				{/if}

				<QuizAnswers
					{answers}
					{results}
					currentQuestion={curQuestion}
					question={quiz.questions[curQuestion - 1].name}
					options={quiz.questions[curQuestion - 1].options}
				/>
				<div class="flex gap-6 p-6">
					{#if curQuestion !== 0}
						<button
							class="btn btn-primary"
							on:click={() => {
								if (curQuestion > 1) curQuestion--;
							}}
						>
							Back
						</button>
					{/if}

					{#if curQuestion !== quiz.questions.length}
						<button
							class="btn btn-primary"
							on:click={() => {
								if (curQuestion < quiz.questions.length) curQuestion++;
							}}
						>
							Next
						</button>
					{/if}

					{#if curQuestion === quiz.questions.length}
						<button class="btn btn-primary" on:click={handleSubmit}>
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
				<!-- Sidebar content here -->
				{#each quiz.questions ?? [] as question, index}
					<li>
						<button on:click={() => (curQuestion = index + 1)}
							>Question {index + 1}</button
						>
					</li>
				{/each}
			</ul>
		</div>
	</div>
{/if}
