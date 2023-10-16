<script lang="ts">
	import { goto } from '$app/navigation';
	import { createQuiz, type Question, type Quiz } from '$lib/api';
	import { isLoggedIn, user } from '$lib/auth';
	import QuizQuestions from '$lib/components/QuizQuestions.svelte';
	import { onMount } from 'svelte';

	onMount(async () => {
		if (!(await isLoggedIn())) return goto('/login');
	});

	function defaultQuestion() {
		return {
			name: '',
			options: [],
			answers: [],
		};
	}

	let loading = false;

	async function handleSubmit() {
		loading = true;
		const response = await createQuiz(q);

		if (response === undefined) {
			loading = false;
			return;
		}

		await goto(`/quizzes/${response}`);
	}

	let q = {
		author: 0,
		title: '',
		description: '',
		questions: [defaultQuestion()] as Question[],
	} satisfies Quiz;
</script>

<div class="flex flex-col gap-6 p-8 items-center w-full">
	<div class="grid items-center gap-2 w-6/12">
		<input
			type="text"
			placeholder="Quiz name..."
			class="input h-14 bg-base-200 w-full text-2xl font-bold"
			bind:value={q.title}
		/>

		<textarea
			class="textarea bg-base-200 w-full h-32"
			placeholder="Quiz description..."
			bind:value={q.description}
		/>
	</div>

	{#each q.questions as _, index}
		<div class="w-6/12">
			<QuizQuestions
				bind:question={q.questions[index]}
				on:remove={() => {
					q.questions.splice(index, 1);
					q = q;
				}}
			/>
		</div>
	{/each}

	<button
		class="w-1/2 bg-base-200 h-32 rounded-2xl hover:bg-base-300 transition-all duration-300 flex flex-row items-center gap-6 justify-center"
		on:click={() => {
			q.questions.push(defaultQuestion());
			q = q;
		}}
	>
		<svg class="fill-current h-8 w-8" viewBox="0 0 24 24"
			><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" /></svg
		>
		<span class="text-2xl">Add another question...</span>
	</button>

	<button
		class="w-1/2 bg-base-200 h-32 rounded-2xl hover:bg-primary transition-all duration-300 flex flex-row items-center gap-6 justify-center"
		disabled={loading || !q.questions.length}
		on:click={handleSubmit}
	>
		<svg class="fill-current h-8 w-8" viewBox="0 0 24 24">
			<path
				d="M9.19 6.35c-2.04 2.29-3.44 5.58-3.57 5.89L2 10.69l4.05-4.05c.47-.47 1.15-.68 1.81-.55l1.33.26zM11.17 17s3.74-1.55 5.89-3.7c5.4-5.4 4.5-9.62 4.21-10.57-.95-.3-5.17-1.19-10.57 4.21C8.55 9.09 7 12.83 7 12.83L11.17 17zm6.48-2.19c-2.29 2.04-5.58 3.44-5.89 3.57L13.31 22l4.05-4.05c.47-.47.68-1.15.55-1.81l-.26-1.33zM9 18c0 .83-.34 1.58-.88 2.12C6.94 21.3 2 22 2 22s.7-4.94 1.88-6.12C4.42 15.34 5.17 15 6 15c1.66 0 3 1.34 3 3zm4-9c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2z"
			/>
		</svg>
		<span class="text-2xl">Create quiz</span>
	</button>
</div>
