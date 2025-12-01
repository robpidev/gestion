<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<nav>
	<ul>
		<li><a href="/dashboard">Dashboard</a></li>
		<li><a href="/dashboard/expenses">Expenses</a></li>
		<li><a href="/dashboard/incomes">Income</a></li>
		<li><a href="/login">login</a></li>
	</ul>
</nav>

{@render children?.()}

<style>
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
