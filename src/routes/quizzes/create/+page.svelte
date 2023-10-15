<script lang="ts">
	import { goto } from "$app/navigation";
	import { createQuiz, type Question, type Quiz } from "$lib/api";
	import QuizQuestions from "$lib/components/QuizQuestions.svelte";

	function defaultQuestion() {
		return {
			name: "",
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
			<QuizQuestions
				bind:question={q.questions[index]}
				on:remove={() => {
					q.questions.splice(index, 1);
					q = q;
				}}
			/>
		{/each}
	</div>
	<button
		class="btn btn-neutral w-full max-w-5xl h-32 text-2xl uppercase"
		on:click={() => {
			q.questions.push(defaultQuestion());
			q = q;
		}}>Add question</button
	>
</div>

<button
	class="btn btn-ghost"
	disabled={loading || !q.questions.length}
	on:click={handleSubmit}>Create Quiz!</button
>
