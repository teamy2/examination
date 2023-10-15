<script lang="ts">
<<<<<<< HEAD
	import { quizzes } from '$lib/quizzes';
=======
	import { quizzes } from "$lib/quizzes";
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();
>>>>>>> 95d6abc (feat: add Quiz creation post functionality)


	export let question: string;
	export let options: string[];
	//export let options: string[]
</script>

<div class="card bg-base-200 shadow-xl w-full">
	<div class="card-body">
		<div class="w-full max-w-xs card-title">{question}</div>

		<div class="grid gap-2">
			{#each { length: options.length } as _, index}
				<div class="flex flex-row items-center">
					<input type="checkbox" class="checkbox col-span-1 checkbox-success" />
					<div> {options}</div>
				</div>
			{/each}
		</div>

		<div class="card-actions justify-end" />
	</div>
	<button class="btn btn-error" on:click={() => dispatch("delete")}>X</button>
</div>
