<script lang="ts">
	import { user } from '$lib/auth';
	import * as api from '$lib/api';
	import { goto } from '$app/navigation';

	let username = '';
	let password = '';

	let error = '';

	async function login() {
		const data = await api.login(username, password);
		if (!data) return (error = 'Invalid username or password');

		$user = data;
		goto('/quizzes');
	}
</script>

<div class="grid place-items-center min-h-screen">
	<form class="card-body" on:submit|preventDefault={login}>
		<div class="form-control">
			<input
				id="username"
				type="text"
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
				id="password"
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
			<button class="btn btn-primary">Login</button>
		</div>
		<div class="form-control">
			<a class="btn btn-primary" href="/register">Create New Account</a>
		</div>
	</form>
</div>
