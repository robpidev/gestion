<script lang="ts">
	import type { Expense } from '$lib/interfaces/expenses';
	import Icon from '@iconify/svelte';
	import { page } from '$app/state';
	import { slide, fly } from 'svelte/transition';

	let monySymbol = '$';

	interface ExpensesGrouped {
		date: string;
		expenses: Expense[];
		total: number;
	}

	interface AccMap {
		[date: string]: {
			expenses: Expense[];
			total: number;
		};
	}

	let { expenses }: { expenses: Expense[] } = $props();

	// let expensedGrouped = $state([] as ExpensesGrouped[]);

	let expensedGrouped = $derived.by(() =>
		Object.entries(
			expenses.reduce((acc: AccMap, item: Expense) => {
				const date = item.date.toLocaleString().split('T')[0];

				if (!acc[date]) acc[date] = { expenses: [], total: 0 };

				acc[date].expenses.push(item);
				acc[date].total += item.amount;
				return acc;
			}, {})
		).map(([date, expenses]) => ({ date, expenses: expenses.expenses, total: expenses.total }))
	);

	// sum amounts

	async function deleteExpense(id: string) {
		let res = await fetch(`${page.url.pathname}/${id}`, {
			method: 'DELETE'
		});
		if (res.ok) {
			expensedGrouped = expensedGrouped.map((expensesGrouped) => {
				const expenses = expensesGrouped.expenses.filter((concept) => concept.id !== id);
				return {
					expenses,
					total: expenses.reduce((acc, concept) => acc + concept.amount, 0),
					date: expensesGrouped.date
				} as ExpensesGrouped;
			});
		}
	}
</script>

<!-- TODO: If date don't have none expenses, don't show -->
<div class="concepts">
	{#each expensedGrouped as daily}
		{#if daily.expenses.length > 0}
			<!-- content here -->
			<div class="daily">
				<span class="date">{daily.date}</span>
				<ul>
					{#each daily.expenses as expense}
						<li out:slide in:fly={{ y: 20 }}>
							<label>
								<input type="checkbox" checked={expense?.processed} />
								{#if expense?.processed}
									<span>Processed</span>
								{:else}
									<span>Not processed</span>
								{/if}
							</label>
							<span class="desc">{expense?.description}</span>

							<span class="amount">{monySymbol} {expense?.amount}</span>
							<!-- <DeleteExpense id={concept?.id} /> -->

							<button class="delete" onclick={() => deleteExpense(expense.id)}>
								<Icon icon="material-symbols:delete-rounded" />
							</button>
						</li>
					{/each}
				</ul>
				<span class="total">Total: {monySymbol} {daily.total}</span>
			</div>
		{/if}
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

	.amount {
		text-align: right;
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

	li {
		border: solid 1px rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		padding: 0rem 1rem;
		gap: 1rem;
		display: grid;
		grid-template-columns: 1fr 2fr 1fr auto;
		justify-content: space-between;
		align-items: center;
	}

	li:hover {
		background: rgba(255, 255, 255, 0.1);
	}
	.delete {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 32px;
		padding: 0.2rem;
		display: flex;
		width: max-content;
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
