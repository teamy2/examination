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
			<label class="label">
				<span class="label-text">Username</span>
			</label>
			<input
				type="email"
				placeholder="email"
				class="input input-bordered"
				required
				bind:value={username}
			/>
		</div>
		<div class="form-control">
			<label class="label">
				<span class="label-text">Password</span>
			</label>
			<input
				type="password"
				placeholder="password"
				class="input input-bordered"
				required
				bind:value={password}
			/>
		</div>
		<div class="form-control mt-6">
			<button class="btn btn-primary" on:click={handleRegister}>Register</button
			>
		</div>
	</form>
</div>
