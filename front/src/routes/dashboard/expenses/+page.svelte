<script>
	import Add from './Add.svelte';
	import Expenses from './Expenses.svelte';

	let { data } = $props();

	// Data from load server
</script>

<Add />

{#if data.expenses}
	<Expenses expenses={data.expenses} />
{:else}
	<p>Loading...</p>
{/if}
<!-- else if content here -->
