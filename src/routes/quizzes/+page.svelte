<script lang="ts">
    import {quizzes} from "$lib/quizzes";
</script>


{#each quizzes as quiz, index}
<div>
    <a href="/quizzes/{quiz.id}">Go to quiz {quiz.title}</a>
</div>
{/each}