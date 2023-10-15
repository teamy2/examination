<script lang="ts">
	let answers = 1;
</script>

<div class="card w-full bg-base-200 shadow-xl">
	<div class="card-body">
		<input
			type="text"
			placeholder="Type Question Here..."
			class="input w-full max-w-xs card-title"
		/>

		<div class="grid gap-2">
			{#each { length: answers } as _, index}
				<div class="flex flex-row items-center">
					<input type="checkbox" class="checkbox col-span-1 checkbox-success" />
					<input
						type="text"
						class="input w-full bg-base-200"
						placeholder="Answer {index + 1}"
					/>
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
