<script lang="ts">
	import { quizzes } from "$lib/quizzes";

	let answers = 4;
</script>

<div class="card w-full bg-base-200 shadow-xl">
	<div class="card-body">
		<div class="input w-full max-w-xs card-title">Type Question Here...</div>

		<div class="grid gap-2">
			{#each { length: answers } as _, index}
				<div class="flex flex-row items-center">
					<input type="checkbox" class="checkbox col-span-1 checkbox-success" />
					<div>Answer {index + 1}</div>
				</div>
			{/each}
		</div>

		<div class="card-actions justify-end">
			<button class="btn btn-primary" on:click={() => answers++}
				>Add Answer</button
			>
		</div>
	</div>
</div>
