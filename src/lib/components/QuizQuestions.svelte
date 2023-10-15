<script lang="ts">
	let answers = 1;
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		<input
			type="text"
			placeholder="Type Question Here..."
			class="input w-full max-w-xs card-title"
		/>

		<div class="grid grid-cols-3 gap-2">
			{#each { length: answers } as _, index}
				<input type="checkbox" class="checkbox col-span-1 checkbox-success" />
				<input
					type="text"
					class="input w-full max-w-xs card-title col-span-2"
					placeholder="Answer {index + 1}"
				/>
			{/each}
		</div>

		<div class="card-actions justify-end">
			<button class="btn btn-primary" on:click={() => answers++}
				>Add Answer</button
			>
		</div>
	</div>
</div>
