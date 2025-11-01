<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="container">
	<nav>
		<ul>
			<li><a href="/">Home</a></li>
			<li><a href="/expenses">Expenses</a></li>
			<li><a href="/incomes">Income</a></li>
		</ul>
	</nav>

	{@render children?.()}
</div>

<style>
	.container {
		width: 100%;
		max-width: 1200px;
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		padding: 12px;
		gap: 12px;
	}

	nav {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1em;
	}

	ul {
		list-style: none;
		display: flex;
		gap: 1em;
		padding: 0;
		justify-content: space-evenly;
	}

	a {
		font-size: 1.2em;
		text-decoration: none;
		color: green;
	}
</style>
