<script lang="ts">
	import type { Question, Quiz } from "$lib/api";
	import QuizQuestions from "$lib/components/QuizQuestions.svelte";

	function defaultQuestion() {
		return {
			name: "",
			options: [],
			answers: [],
		};
	}

	let q = {
		author: "",
		title: "",
		description: "",
		questions: [defaultQuestion()] as Question[],
	} satisfies Quiz;
</script>

<div class="pb-5">
	<input
		type="text"
		placeholder="Quiz Name"
		class="input input-bordered input-info w-full max-w-xs"
		bind:value={q.title}
	/>
	<label class="btn btn-info" for="bg-select">Choose Thumbnail</label>
	<input id="bg-select" type="file" class="file-input w-full max-w-xs" hidden />
</div>

<div class="flex justify-center gap-8 flex-col place-items-center">
	<div class="grid gap-3 max-w-5xl w-full">
		{#each q.questions as _, index}
			<QuizQuestions bind:question={q.questions[index]} />
		{/each}
	</div>
	<button
		class="btn btn-neutral w-full max-w-5xl h-32"
		on:click={() => {
			q.questions.push(defaultQuestion());
			q = q;
		}}>Add Question!</button
	>
</div>
