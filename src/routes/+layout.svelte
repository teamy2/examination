<!-- @format -->
<script lang="ts">
	import { page } from "$app/stores";
	import "../app.css";
</script>

{#if $page.url.pathname === '/login' || $page.url.pathname === '/'}
	<slot />
{:else}
	<div class="min-h-screen">
		<div class="navbar">
			<div class="flex-1">
				<a href="/" class="btn btn-ghost normal-case text-xl red">Examination</a
				>
			</div>
		</div>
		<div class="h-full p-8">
			<slot />
		</div>
	</div>
{/if}
