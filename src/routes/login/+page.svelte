<script lang="ts">
	import { user } from '$lib/auth';
	import * as api from '$lib/api';

	let username = '';
	let password = '';

	let error = '';

	async function login() {
		const data = await api.login(username, password);
		if (!data) return (error = 'Invalid username or password');

		$user = data;
	}
</script>

<div class="grid place-items-center min-h-screen">
	<form class="card-body" on:submit|preventDefault={login}>
		<div class="form-control">
			<label class="label" for="username">
				<span class="label-text">Username</span>
			</label>
			<input
				id="username"
				type="text"
				placeholder="Username"
				class="input input-bordered"
				required
				bind:value={username}
			/>
			<label class="label" for="username">
				<span class="label-text">Username</span>
			</label>
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
			<button class="btn btn-primary">Login</button>
		</div>
	</form>
</div>
