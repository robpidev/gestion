<script lang="ts">
	import Icon from '@iconify/svelte';

	let monySymbol = '$';

	let { concepts = [] }: { concepts: any[] } = $props();

	let gruped = Object.entries(
		concepts.reduce((acc, item) => {
			const date = item.date.toLocaleString().split(', ')[0];

			if (!acc[date]) acc[date] = { items: [], total: 0 };

			acc[date].items.push(item);
			acc[date].total += item.amount;
			return acc;
		}, {})
	).map(([date, concepts]) => ({ date, concepts: concepts.items, total: concepts.total })) as {
		date: string;
		concepts: any[];
		total: number;
	}[];

	// sum amounts
	console.log(gruped);
</script>

<div class="concepts">
	{#each gruped as daily}
		<div class="daily">
			<span class="date">{daily.date}</span>
			<ul>
				{#each daily.concepts as concept}
					<li>
						<div>
							<span>{concept?.description}</span>
							<span>{monySymbol} {concept?.amount}</span>
						</div>
						<button>
							<Icon icon="mingcute:delete-fill" />
						</button>
					</li>
				{/each}
			</ul>
			<span class="total">Total: {monySymbol} {daily.total}</span>
		</div>
	{/each}
</div>

<style>
	.concepts {
		width: 100%;
		display: flex;
		gap: 1rem;
		border-radius: 6px;
		overflow: auto;
		flex-wrap: wrap;
	}

	.daily {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		border: solid 1px rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		padding: 1rem;
	}

	.date,
	.total {
		margin-left: auto;
		width: max-content;
		text-align: right;
		padding: 0.2rem 1rem;
		border-radius: 6px;
	}

	.date {
		background: var(--color-bg-primary);
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
		width: 100%;
	}

	ul div {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	li {
		border: solid 1px rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		padding: 0rem 1rem;
		gap: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	li:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	button {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 32px;
		padding: 0.2rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	button:hover {
		background: var(--color-bg-warning);
	}

	button:active {
		box-shadow: 0 0 0 2px var(--color-border-focus);
	}
</style>
