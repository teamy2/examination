<script>
	import Quiz from '../../lib/components/Quiz.svelte';
</script>

<div style="display: flex;">
	<div class="avatar p-10">
		<div class="w-80 h-30">
			<img src="https://i.stack.imgur.com/l60Hf.png" />
		</div>
	</div>
	<div style="padding: 50px">
		<h1 style="font-size: 50px">Username</h1>
		<h1>Description</h1>
	</div>
</div>

<div class="align-middle justify-center content-center text-center">
	<h1 class="card-body text-3xl border-b-2 justify-center">My Quizzes</h1>
</div>
<div class="flex gap-10 justify-center border-spacing-y-6 p-10">
	<div class="card w-96 bg-base-100 shadow-xl image-full">
		<figure>
			<img
				src="/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg"
				alt="Shoes"
			/>
		</figure>
		<div class="card-body">
			<h2 class="card-title">Title</h2>
			<p>Description</p>
			<div class="card-actions justify-end">
				<button class="btn btn-primary">Edit</button>
			</div>
		</div>
	</div>
	<Quiz title="Title1" description="desc1" completed={true} />
</div>

<div>
	<h1 class="card-body text-3xl border-b-2 text-center">Completed Quizzes</h1>
</div>
<div class="flex gap-10 justify-center border-spacing-y-6 p-10">
	<div class="card w-96 bg-base-100 shadow-xl image-full">
		<figure>
			<img
				src="/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg"
				alt="Shoes"
			/>
		</figure>
		<div class="card-body">
			<h2 class="card-title">Shoes!</h2>
			<p>If a dog chews shoes whose shoes does he choose?</p>
			<div class="card-actions justify-end">
				<button class="btn btn-primary">View Results</button>
			</div>
		</div>
	</div>
	<Quiz title="Title2" description="desc2" completed={false} />
</div>
