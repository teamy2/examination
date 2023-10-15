<script lang="ts">
	import { page } from '$app/stores';
	import '../app.css';
</script>

{#if $page.url.pathname === '/login'}
	<slot />
{:else}
	<div class="min-h-screen">
		<div class="navbar">
			<div class="flex-1">
				<a class="btn btn-ghost normal-case text-xl red">Examination</a>
			</div>
		</div>
		<div class="h-full p-8">
			<slot />
		</div>
	</div>
{/if}
