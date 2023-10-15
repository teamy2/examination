<script lang="ts">

    let questionsAmount = 1;
</script>




<div class="pb-5">
    Create a Quiz!
    <input type="text" placeholder="Quiz Name" class="input input-bordered input-info w-full max-w-xs" />
    <label class="btn btn-info" for="bg-select">Choose Background</label>
    <input id="bg-select" type="file" class="file-input w-full max-w-xs" hidden/>
</div>

<div class="flex justify-center text-center">
<div class="grid grid-cols-3 gap-3">
    {#each {length : questionsAmount} as _, index}
    <input type="text" placeholder="Question {index+1}" class="input input-bordered input-success w-full max-w-xs" />
    <input type="text" placeholder="Answer" class="input input-bordered input-error w-full max-w-xs" />
    <input type="file" class="file-input file-input-bordered file-input-primary w-full max-w-xs" />
    {/each}
    <button  class="btn btn-success"on:click={() => questionsAmount++}>Add Question!</button>
</div>

</div>
