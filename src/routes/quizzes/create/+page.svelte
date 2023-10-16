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

<div class="flex flex-col gap-4 p-8 items-center">
	<div class="flex items-center gap-8">
		<input
			type="text"
			placeholder="Quiz Name"
			class="input input-bordered w-40 max-w-ws"
			bind:value={q.title}
		/>

		<textarea
			class="textarea textarea-bordered"
			placeholder="Description"
			bind:value={q.description}
		/>
	</div>

	{#each q.questions as _, index}
		<div class="py-4 w-6/12">
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
		class="btn"
		on:click={() => {
			q.questions.push(defaultQuestion());
			q = q;
		}}>Add question</button
	>

	<button
		class="btn"
		disabled={loading || !q.questions.length}
		on:click={handleSubmit}>Create Quiz!</button
	>
</div>
