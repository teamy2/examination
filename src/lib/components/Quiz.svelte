<script lang="ts">
	import type { Quiz } from '$lib/api';

	export let completed = false;
	export let quiz: Quiz;

	function randomPastelColour() {
		// create random nice light-ish pastel colours
		const hue = Math.floor(Math.random() * 360);
		const saturation = Math.floor(Math.random() * 30) + 70;
		const lightness = Math.floor(Math.random() * 20) + 80;

		return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
	}
</script>

<a href="/quizzes/{quiz.id}">
	<div
		class="card w-full bg-base-300 shadow-xl image-full hover:scale-105 transition-all duration-300 cursor-pointer min-h-[20rem]"
		style="background-color: {randomPastelColour()};"
	>
		<div class="card-body">
			{#if completed}
				<svg
					class="z-10 absolute top-6 right-6"
					fill="gold"
					xmlns="http://www.w3.org/2000/svg"
					width="24"
					height="24"
					viewBox="0 0 24 24"
				>
					<path
						d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z"
					/>
				</svg>
			{/if}

			<h2 class="card-title asd">{quiz.title}</h2>
			<p class="line-clamp-5 whitespace-pre-line">{quiz.description}</p>
		</div>
	</div>
</a>
