<script lang="ts">
	import { quizzes } from '$lib/quizzes';


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
</div>
