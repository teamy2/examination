<!-- @format -->
<script lang="ts">
	import { getQuizzes, type Quiz } from '$lib/api';
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
		{#each quizzes as quiz, index}
			<a href="/quizzes/{quiz.id}">
				<div
					class="card w-full bg-base-100 image-full hover:scale-105 transition-all duration-300"
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
</div>
