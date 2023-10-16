<script lang="ts">
	export let question: string;
	export let options: string[];
	export let answers: Set<number>[];
	export let currentQuestion: number;
	export let results: boolean[][] | undefined;

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

<div class="card bg-base-300 shadow-xl w-full">
	<div class="card-body relative">
		<div class="absolute top-6 right-6 text-xl">#{currentQuestion + 1}</div>
		<div class="w-full max-w-xs text-3xl pb-6">{question}</div>

		<div class="grid gap-3">
			{#each { length: options.length } as _, index}
				<div class="flex flex-row items-center gap-2">
					<input
						type="checkbox"
						class="checkbox col-span-1"
						on:click={() => handleCheck(currentQuestion, index)}
						checked={answers[currentQuestion]?.has(index)}
						class:checkbox-success={results && results[currentQuestion][index]}
						class:checkbox-error={results && !results[currentQuestion][index]}
					/>
					<div>{options[index]}</div>
				</div>
			{/each}
		</div>
	</div>
</div>
