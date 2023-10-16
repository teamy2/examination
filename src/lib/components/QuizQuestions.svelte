<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let answersAmount = 1;

	export let question = {
		name: '',
		options: [] as string[],
		answers: [] as number[],
	};
</script>

<div class="card w-full bg-base-200 shadow-xl">
	<div class="card-body">
		<button
			on:click={() => dispatch('remove')}
			class="btn btn-md btn-circle btn-ghost absolute right-2 top-2 text-2xl"
			>✕</button
		>
		<input
			type="text"
			placeholder="Question"
			class="input w-6/12 card-title my-4"
			bind:value={question.name}
		/>

		{#each { length: answersAmount } as _, index}
			<div class="flex flex-row items-center gap-4">
				<input
					type="checkbox"
					class="checkbox col-span-1 checkbox-success"
					checked={question.answers.includes(index)}
					on:click={() => {
						question.answers.includes(index)
							? question.answers.splice(question.answers.indexOf(index), 1)
							: question.answers.push(index);
						console.log(question.answers);
					}}
				/>
				<input
					type="text"
					class="input bg-base-200 w-6/12"
					placeholder="Answer {index + 1}"
					bind:value={question.options[index]}
				/>
			</div>
		{/each}

		<div class="card-actions justify-end">
			<button class="btn btn-primary my-4" on:click={() => answersAmount++}
				>Add Answer</button
			>
		</div>
	</div>
</div>
