<script lang="ts">
	import { login } from '$lib/api';
	import { quizzes } from '$lib/quizzes';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let question: string;
	export let options: string[];
	export let answers: Set<number>[];
	export let currentQuestion: number;
	export let results: boolean[][] | undefined;

	const checks: boolean[] = [];

	function handleCheck(currentQuestion: number, answer: number) {
		if (!answers[currentQuestion]) {
			answers[currentQuestion] = new Set();
		}

		if (answers[currentQuestion].has(answer)) {
			answers[currentQuestion].delete(answer);
		} else {
			answers[currentQuestion].add(answer);
		}
	}
</script>

<div class="card bg-base-200 shadow-xl w-full">
	<div class="card-body">
		<div class="w-full max-w-xs card-title">{question}</div>

		<div class="grid gap-2">
			{#each { length: options.length } as _, index}
				<div class="flex flex-row items-center">
					<input
						type="checkbox"
						class="checkbox col-span-1"
						on:click={() => handleCheck(currentQuestion - 1, index)}
						checked={checks[index]}
						class:checkbox-success={results &&
							results[currentQuestion - 1][index]}
						class:checkbox-error={results &&
							!results[currentQuestion - 1][index]}
					/>
					<div>{options[index]}</div>
				</div>
			{/each}
		</div>

		<div class="card-actions justify-end" />
	</div>
</div>
