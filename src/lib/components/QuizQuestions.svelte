<script lang="ts">
	let answersAmount = 1;

	export let question = {
		name: "",
		options: [] as string[],
		answers: [] as number[],
	};
</script>

<div class="card w-full bg-base-200 shadow-xl">
	<div class="card-body">
		<input
			type="text"
			placeholder="Type Question Here..."
			class="input w-full max-w-xs card-title"
			bind:value={question.name}
		/>

		<div class="grid gap-2">
			{#each { length: answersAmount } as _, index}
				<div class="flex flex-row items-center">
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
						class="input w-full bg-base-200"
						placeholder="Answer {index + 1}"
						bind:value={question.options[index]}
					/>
				</div>
			{/each}
		</div>

		<div class="card-actions justify-end">
			<button class="btn btn-primary" on:click={() => answersAmount++}
				>Add Answer</button
			>
		</div>
	</div>
</div>
