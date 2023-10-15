<script lang="ts">
	import { page } from '$app/stores';

	import { quizzes } from '$lib/quizzes';

	const id = parseInt($page.params.id);

	const quiz = quizzes.find((q) => q.id === id)!;
</script>

<div>{quiz.author}</div>
