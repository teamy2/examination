<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let answersAmount = 1;

	export let question = {
		name: '',
		options: [''] as string[],
		answers: [] as number[],
	};
</script>

<div class="card w-full bg-base-200 shadow-xl">
	<div class="card-body">
		<button
			on:click={() => dispatch('remove')}
			class="btn btn-md btn-circle btn-ghost absolute right-2 top-2 text-xl"
			>✕</button
		>
		<input
			type="text"
			placeholder="Question"
			class="input w-6/12 card-title mb-4"
			bind:value={question.name}
		/>

		{#each { length: answersAmount } as _, index}
			<div class="flex flex-row items-center gap-2 pl-2">
				<input
					type="checkbox"
					class="checkbox col-span-1"
					checked={question.answers.includes(index)}
					on:click={() =>
						question.answers.includes(index)
							? question.answers.splice(question.answers.indexOf(index), 1)
							: question.answers.push(index)}
				/>
				<input
					type="text"
					class="input bg-base-200 w-6/12"
					placeholder="Insert option..."
					bind:value={question.options[index]}
				/>
			</div>
		{/each}
		<button
			class="flex flex-row items-center gap-6 cursor-pointer bg-base-300 hover:bg-base-100 transition-all duration-300 h-12 p-2 rounded-lg"
			on:click={() => {
				question.options[answersAmount] = '';
				answersAmount++;
			}}
		>
			<svg class="fill-current h-6 w-6" viewBox="0 0 24 24"
				><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" /></svg
			>
			<p class="text-left">Add another answer...</p>
		</button>
	</div>
</div>
