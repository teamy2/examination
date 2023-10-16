<!-- @format -->
<script lang="ts">
	import { page } from '$app/stores';
	import { ready, user } from '$lib/auth';
	import { onMount } from 'svelte';

	import '../app.css';
	import { goto } from '$app/navigation';

	onMount(() => {
		const u = localStorage.getItem('user');
		if (u) user.set(JSON.parse(u));

		user.subscribe(u => {
			if (u) localStorage.setItem('user', JSON.stringify(u));
			else localStorage.removeItem('user');
		});

		ready.set(true);
	});
</script>

{#if $page.url.pathname === '/login' || $page.url.pathname === '/' || $page.url.pathname === '/register'}
	<slot />
{:else}
	<div class="min-h-screen">
		<div class="navbar bg-base-200">
			<div class="flex-1">
				<a href="/" class="btn btn-ghost normal-case text-xl red">Examination</a
				>
				{#if $user}
				<button class="btn ml-auto mr-6" on:click={() => {
					$user = undefined;
					goto("/login")
				}}>
					Logout
				</button>
				<a href="/profile" class="mr-6">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						class="fill-current"
						><path
							d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm7.753 18.305c-.261-.586-.789-.991-1.871-1.241-2.293-.529-4.428-.993-3.393-2.945 3.145-5.942.833-9.119-2.489-9.119-3.388 0-5.644 3.299-2.489 9.119 1.066 1.964-1.148 2.427-3.393 2.945-1.084.25-1.608.658-1.867 1.246-1.405-1.723-2.251-3.919-2.251-6.31 0-5.514 4.486-10 10-10s10 4.486 10 10c0 2.389-.845 4.583-2.247 6.305z"
						/></svg
					>
				</a>
				{:else}
				<a href="/profile" class="mr-6 ml-auto">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						class="fill-current"
						><path
							d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm7.753 18.305c-.261-.586-.789-.991-1.871-1.241-2.293-.529-4.428-.993-3.393-2.945 3.145-5.942.833-9.119-2.489-9.119-3.388 0-5.644 3.299-2.489 9.119 1.066 1.964-1.148 2.427-3.393 2.945-1.084.25-1.608.658-1.867 1.246-1.405-1.723-2.251-3.919-2.251-6.31 0-5.514 4.486-10 10-10s10 4.486 10 10c0 2.389-.845 4.583-2.247 6.305z"
						/></svg
					>
				</a>
				{/if}
				
			</div>
		</div>
		<div class="h-full">
			<slot />
		</div>
	</div>
{/if}
