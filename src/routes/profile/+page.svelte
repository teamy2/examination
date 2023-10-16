<script lang="ts">
	import { isLoggedIn, user } from '$lib/auth';
	import { onMount } from 'svelte';
	import QuizComponent from '../../lib/components/Quiz.svelte';
	import { goto } from '$app/navigation';
	import { getCompletedQuizzes, getCreatedQuizzes, type Quiz } from '$lib/api';

	let loading = true;

	let completed: Quiz[] = [];
	let created: Quiz[] = [];

	onMount(async () => {
		if (!(await isLoggedIn())) return goto('/login');

		completed = await getCompletedQuizzes($user?.id ?? 0);
		created = await getCreatedQuizzes($user?.id ?? 0);
		loading = false;
	});
</script>

{#if loading}
	<div class="full-screen">
		<span class="loading loading-dots loading-lg" />
	</div>
{:else}
	<div class="flex">
		<div class="avatar p-10">
			<div class="w-80 h-30 rounded-full">
				<img
					src="https://i.stack.imgur.com/l60Hf.png"
					alt=""
					class="object-cover"
				/>
			</div>
		</div>
		<div style="padding: 50px">
			<h1 style="font-size: 50px">{$user?.username}</h1>
			<h1>Description</h1>
		</div>
	</div>

	<div class="align-middle justify-center content-center text-center">
		<h1 class="card-body text-3xl justify-center">My Quizzes</h1>
	</div>
	<div class="flex gap-10 justify-center border-spacing-y-6 p-10">
		{#each created as quiz}
			<QuizComponent {quiz} />
		{/each}
	</div>

	<div>
		<h1 class="card-body text-3xl text-center">Completed Quizzes</h1>
	</div>
	<div class="flex gap-10 justify-center border-spacing-y-6 p-10">
		{#each completed as quiz}
			<QuizComponent {quiz} completed />
		{/each}
	</div>
{/if}

<style>
	.full-screen {
		@apply grid place-items-center items-center w-full min-h-screen;
	}
</style>
