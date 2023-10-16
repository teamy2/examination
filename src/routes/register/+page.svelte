<script>
	import { goto } from '$app/navigation';
	import { register } from '$lib/api';

	let username = '';
	let password = '';
	let error = '';

	async function handleRegister() {
		const response = await register(username, password);
		console.log(response);
		if (!response) return (error = 'Failed to register');
		goto('/login');
	}
</script>

<div class="grid place-items-center min-h-screen">
	<form class="card-body">
		<div class="form-control">
			<input
				type="username"
				placeholder="Username"
				class="input input-bordered"
				required
				bind:value={username}
			/>
			{#if error}
				<label class="label" for="username">
					<span class="label-text text-error">{error}</span>
				</label>
			{/if}
		</div>
		<div class="form-control">
			<input
				type="password"
				placeholder="●●●●●●●●"
				class="input input-bordered"
				required
				bind:value={password}
			/>
			{#if error}
				<label class="label" for="password">
					<span class="label-text text-error">{error}</span>
				</label>
			{/if}
		</div>
		<div class="form-control mt-6">
			<button class="btn btn-primary" on:click={handleRegister}>Register</button
			>
		</div>
		<div class="form-control">
			<a class="btn btn-primary" href="/login">Already Have an Account?</a>
		</div>
	</form>
</div>
