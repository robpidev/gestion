<script lang="ts">
	import AddIncome from './AddIncome.svelte';
	import Incomes from './Incomes.svelte';

	let { data } = $props();
</script>

<AddIncome />

{#if data.incomes}
	<Incomes incomes={data.incomes} />
{:else}
	<div class="loading">
		<p>Loading incomes...</p>
	</div>
{/if}

<style>
	.loading {
		display: flex;
		justify-content: center;
		padding: 2rem;
		color: var(--color-text-secondary);
	}
</style>
