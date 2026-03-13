<script lang="ts">
	import type { Income } from '$lib/interfaces/incomes';
	import Icon from '@iconify/svelte';
	import { page } from '$app/state';
	import { slide, fly } from 'svelte/transition';
	import { invalidateAll } from '$app/navigation';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';

	let monySymbol = '$';

	interface IncomesGrouped {
		date: string;
		incomes: Income[];
		total: number;
	}

	interface AccMap {
		[date: string]: {
			incomes: Income[];
			total: number;
		};
	}

	let { incomes }: { incomes: Income[] } = $props();

	let incomesGrouped = $derived.by(() =>
		Object.entries(
			incomes.reduce((acc: AccMap, item: Income) => {
				const date = item.date.toLocaleString().split('T')[0];
				if (!acc[date]) acc[date] = { incomes: [], total: 0 };
				acc[date].incomes.push(item);
				acc[date].total += item.amount;
				return acc;
			}, {})
		)
			.map(([date, group]) => ({
				date,
				incomes: group.incomes,
				total: group.total
			}))
			.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
	);

	let editingId = $state<string | null>(null);
	let editData = $state({ description: '', amount: 0, processed: false, date: '' });

	// Modal State
	let showDeleteModal = $state(false);
	let incomeToDelete = $state<string | null>(null);

	function startEditing(income: Income) {
		editingId = income.id;
		editData = {
			description: income.description,
			amount: income.amount,
			processed: income.processed,
			date: new Date(income.date).toISOString().split('T')[0]
		};
	}

	function cancelEditing() {
		editingId = null;
	}

	async function saveEdit(id: string) {
		const formData = new FormData();
		formData.append('description', editData.description);
		formData.append('amount', editData.amount.toString());
		formData.append('processed', editData.processed.toString());
		formData.append('date', editData.date);

		let res = await fetch(`${page.url.pathname}/${id}`, {
			method: 'PATCH',
			body: formData
		});

		if (res.ok) {
			editingId = null;
			await invalidateAll();
		}
	}

	function confirmDelete(id: string) {
		incomeToDelete = id;
		showDeleteModal = true;
	}

	async function handleDelete() {
		if (!incomeToDelete) return;
		
		let res = await fetch(`${page.url.pathname}/${incomeToDelete}`, {
			method: 'DELETE'
		});
		
		if (res.ok) {
			showDeleteModal = false;
			incomeToDelete = null;
			await invalidateAll();
		}
	}

	function formatDate(dateStr: string) {
		const date = new Date(dateStr);
		return new Intl.DateTimeFormat('en-US', {
			weekday: 'long',
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		}).format(date);
	}
</script>

<div class="incomes-list">
	{#each incomesGrouped as daily}
		{#if daily.incomes.length > 0}
			<section class="daily-group" transition:slide>
				<header class="group-header">
					<div class="date-info">
						<Icon icon="material-symbols:calendar-today-outline" />
						<span class="date-text">{formatDate(daily.date)}</span>
					</div>
					<div class="group-total">
						<span class="total-label">Daily Total:</span>
						<span class="total-amount">{monySymbol}{daily.total.toFixed(2)}</span>
					</div>
				</header>

				<div class="incomes-container">
					{#each daily.incomes as income (income.id)}
						<div class="income-row" out:slide={{ duration: 300 }} in:fly={{ y: 20, duration: 400 }}>
							{#if editingId === income.id}
								<div class="edit-mode">
									<div class="edit-inputs">
										<div class="status-col">
											<label class="edit-toggle">
												<input type="checkbox" bind:checked={editData.processed} />
												<span class="edit-slider"></span>
											</label>
										</div>
										<input type="text" bind:value={editData.description} placeholder="Description" />
										<input type="number" bind:value={editData.amount} step="0.01" placeholder="Amount" />
										<input type="date" bind:value={editData.date} />
									</div>
									<div class="edit-actions">
										<button class="save-btn" onclick={() => saveEdit(income.id)} title="Save">
											<Icon icon="material-symbols:check-rounded" />
										</button>
										<button class="cancel-btn" onclick={cancelEditing} title="Cancel">
											<Icon icon="material-symbols:close-rounded" />
										</button>
									</div>
								</div>
							{:else}
								<div class="status-col">
									<div class="status-indicator" class:processed={income.processed} title={income.processed ? 'Received' : 'Pending'}>
										<Icon icon={income.processed ? 'material-symbols:check-circle' : 'material-symbols:pending'} />
									</div>
								</div>

								<div class="description-col">
									<span class="description">{income.description}</span>
									<span class="status-text">{income.processed ? 'Received' : 'Pending'}</span>
								</div>

								<div class="amount-col">
									<span class="amount">{monySymbol}{income.amount.toFixed(2)}</span>
								</div>

								<div class="actions-col">
									<button class="edit-btn" onclick={() => startEditing(income)} aria-label="Edit income">
										<Icon icon="material-symbols:edit-outline-rounded" />
									</button>
									<button class="delete-btn" onclick={() => confirmDelete(income.id)} aria-label="Delete income">
										<Icon icon="material-symbols:delete-outline-rounded" />
									</button>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</section>
		{/if}
	{:else}
		<div class="empty-state">
			<Icon icon="material-symbols:account-balance-wallet-outline" />
			<p>No incomes recorded yet.</p>
		</div>
	{/each}

	<ConfirmModal 
		show={showDeleteModal} 
		title="Delete Income"
		message="Are you sure you want to delete this income? This action cannot be undone."
		onConfirm={handleDelete}
		onCancel={() => { showDeleteModal = false; incomeToDelete = null; }}
	/>
</div>

<style>
	.incomes-list { display: flex; flex-direction: column; gap: 2.5rem; width: 100%; }
	.daily-group { background-color: #1e1e1e; border-radius: 12px; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.05); box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15); }
	.group-header { padding: 1.25rem 1.5rem; background-color: rgba(255, 255, 255, 0.03); display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(255, 255, 255, 0.05); }
	.date-info { display: flex; align-items: center; gap: 0.75rem; color: var(--color-bg-primary); }
	.date-text { font-weight: 600; font-size: 1rem; color: white; }
	.group-total { display: flex; align-items: baseline; gap: 0.5rem; }
	.total-label { font-size: 0.85rem; color: var(--color-text-secondary); }
	.total-amount { font-size: 1.1rem; font-weight: 700; color: var(--color-bg-primary); }
	.incomes-container { display: flex; flex-direction: column; }
	
	.income-row {
		display: grid;
		grid-template-columns: auto 1fr auto auto;
		align-items: center;
		padding: 1rem 1.5rem;
		gap: 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.03);
		transition: background-color 0.2s;
	}

	.edit-mode {
		display: flex;
		width: 100%;
		gap: 1rem;
		align-items: center;
		padding: 0.5rem 0;
	}

	.edit-inputs {
		display: flex;
		flex: 1;
		gap: 0.5rem;
		align-items: center;
	}

	.edit-inputs input {
		background: #2a2a2a;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		color: white;
		padding: 0.4rem 0.6rem;
		font-size: 0.9rem;
	}

	.edit-inputs input[type="text"] { flex: 2; }
	.edit-inputs input[type="number"] { flex: 1; width: 80px; }
	.edit-inputs input[type="date"] { flex: 1; }

	/* Styled Toggle for Edit Mode */
	.edit-toggle {
		position: relative;
		display: inline-block;
		width: 36px;
		height: 20px;
		flex-shrink: 0;
	}

	.edit-toggle input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.edit-slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: #444;
		transition: 0.4s;
		border-radius: 20px;
	}

	.edit-slider:before {
		position: absolute;
		content: '';
		height: 14px;
		width: 14px;
		left: 3px;
		bottom: 3px;
		background-color: white;
		transition: 0.4s;
		border-radius: 50%;
	}

	.edit-toggle input:checked + .edit-slider {
		background-color: var(--color-bg-primary);
	}

	.edit-toggle input:checked + .edit-slider:before {
		transform: translateX(16px);
	}

	.actions-col, .edit-actions {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.save-btn, .cancel-btn, .edit-btn, .delete-btn {
		background: none;
		border: none;
		color: var(--color-text-secondary);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 6px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
		font-size: 1.25rem;
	}

	.save-btn:hover { color: var(--color-bg-primary); background: rgba(29, 185, 84, 0.1); }
	.cancel-btn:hover { color: #ff4d4d; background: rgba(255, 77, 77, 0.1); }
	.edit-btn:hover { color: var(--color-bg-primary); background: rgba(29, 185, 84, 0.1); }
	.delete-btn:hover { background-color: rgba(255, 77, 77, 0.1); color: #ff4d4d; }

	@media (max-width: 640px) {
		.income-row {
			grid-template-columns: auto 1fr auto;
			grid-template-areas: "status desc amount" "empty empty actions";
			gap: 0.75rem;
			padding: 1rem;
		}
		.status-col { grid-area: status; }
		.description-col { grid-area: desc; }
		.amount-col { grid-area: amount; text-align: right; }
		.actions-col { grid-area: actions; justify-self: end; margin-top: -0.5rem; }
		.group-header { padding: 1rem; flex-direction: column; align-items: flex-start; gap: 0.5rem; }
		.group-total { width: 100%; justify-content: space-between; }
	}

	.income-row:last-child { border-bottom: none; }
	.income-row:hover { background-color: rgba(255, 255, 255, 0.02); }
	.status-indicator { font-size: 1.5rem; color: #888; display: flex; }
	.status-indicator.processed { color: var(--color-bg-primary); }
	.description-col { display: flex; flex-direction: column; gap: 0.25rem; }
	.description { font-weight: 500; color: white; }
	.status-text { font-size: 0.75rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em; }
	.amount-col .amount { font-family: 'JetBrains Mono', monospace; font-weight: 600; font-size: 1.1rem; }
	.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 4rem; color: var(--color-text-secondary); gap: 1rem; background-color: #1e1e1e; border-radius: 12px; border: 1px dashed rgba(255, 255, 255, 0.1); }
	.empty-state :global(svg) { font-size: 3rem; opacity: 0.3; }
</style>
