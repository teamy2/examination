<script lang="ts">
	import { page } from '$app/stores';
	import { getQuiz, type Quiz } from '$lib/api';

	import QuizAnswers from '$lib/components/QuizAnswers.svelte';

	import { onMount } from 'svelte';

	const id = parseInt($page.params.id);

	let quiz: Quiz;
	let curQuestion = 1;

	onMount(async () => {
		quiz = await getQuiz(id);
	});
</script>

{#if quiz}
	<div class="drawer lg:drawer-open">
		<input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col">
			<!-- Page content here -->
			<div class="h-20 content-center">
				<h1 class="top-8 text-3xl">{quiz.title}</h1>
				<h1>
					By: {quiz.author}
				</h1>
				<QuizAnswers
					question={quiz.questions[curQuestion - 1].name}
					options={quiz.questions[curQuestion - 1].options}
				/>
				<div class="flex gap-6 p-6">
					<button
						class="btn btn-primary"
						on:click={() => {
							if (curQuestion > 1) curQuestion--;
						}}
					>
						Back
					</button>
					<button
						class="btn btn-primary"
						on:click={() => {
							if (curQuestion < quiz.questions.length) curQuestion++;
						}}
					>
						Next
					</button>
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
