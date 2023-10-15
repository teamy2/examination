<script lang="ts">
	let questionsAmount = 1;

	import QuizQuestions from "$lib/components/QuizQuestions.svelte";
</script>

<div class="pb-5">
	Create a Quiz!
	<input
		type="text"
		placeholder="Quiz Name"
		class="input input-bordered input-info w-full max-w-xs"
	/>
	<label class="btn btn-info" for="bg-select">Choose Background</label>
	<input id="bg-select" type="file" class="file-input w-full max-w-xs" hidden />
</div>

<div class="flex justify-center gap-8 flex-col place-items-center">
	<div class="grid gap-3 max-w-5xl w-full">
		{#each { length: questionsAmount } as _, index}
			<QuizQuestions />
		{/each}
	</div>
	<button
		class="btn btn-neutral w-full max-w-5xl h-32"
		on:click={() => questionsAmount++}>Add Question!</button
	>
</div>
