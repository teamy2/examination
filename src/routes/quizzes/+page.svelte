<!-- @format -->
<script lang="ts">
	import { getQuizzes, type Quiz } from '$lib/api';
	import { onMount } from 'svelte';

	let quizzes: Quiz[] = [];
	onMount(async () => {
		quizzes = await getQuizzes();
	});
</script>

<h1 class="text-center text-3xl py-8">Available Quizzes</h1>
<div class="flex flex-row flex-wrap justify-center">
	{#each quizzes as quiz, index}
		<a href="/quizzes/{quiz.id}">
			<div
				class="card w-96 bg-base-100 image-full p-8 hover:scale-105 transition-all duration-300"
			>
				<figure>
					<img
						src="https://images.unsplash.com/photo-1505751172876-fa1923c5c528?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2070&q=80"
						alt="Shoes"
					/>
				</figure>
				<div class="card-body">
					<h2 class="card-title">{quiz.title}</h2>
					<p>{quiz.description}</p>
					<div class="card-actions justify-center" />
				</div>
			</div>
			<div />
		</a>
	{/each}
</div>
