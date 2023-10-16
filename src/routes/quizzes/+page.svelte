<!-- @format -->
<script lang="ts">
	import { getQuizzes, type Quiz } from '$lib/api';
	import QuizComponent from '$lib/components/Quiz.svelte';
	import { onMount } from 'svelte';

	let quizzes: Quiz[] = [];
	onMount(async () => {
		quizzes = await getQuizzes();
	});
</script>

<div class="flex-column text-center">
	<h1 class="text-center text-6xl py-8">Available Quizzes</h1>
	<a href="/quizzes/create" class="btn btn-lg"> Create a quiz </a>
</div>

<div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
	<div class="grid md:grid-cools-2 lg:grid-cols-3 gap-6">
		{#each quizzes as quiz}
			<QuizComponent {quiz} />
		{/each}
	</div>
</div>
