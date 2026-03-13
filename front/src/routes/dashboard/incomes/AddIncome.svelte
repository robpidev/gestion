<script lang="ts">
	import Icon from '@iconify/svelte';
	import { enhance } from '$app/forms';

	interface FormData {
		processed: boolean;
		description: string;
		amount: number;
		date: string;
	}
	let formdata = $state({
		processed: false,
		description: '',
		amount: 0,
		date: new Date().toISOString().split('T')[0]
	} as FormData);

	let disabled: boolean = $derived.by(
		() => !formdata.amount || !formdata.description || !formdata.date
	);

	let sending = $state(false);
</script>

<div class="add-income-container">
	<form
		method="POST"
		action="?/create"
		use:enhance={() => {
			sending = true;
			return async ({ update }) => {
				sending = false;
				await update();
				formdata.description = '';
				formdata.amount = 0;
				formdata.processed = false;
			};
		}}
	>
		<div class="form-grid">
			<div class="input-group">
				<label for="description">Source / Description</label>
				<input
					id="description"
					bind:value={formdata.description}
					name="description"
					type="text"
					placeholder="Salary, Freelance, etc."
					required
				/>
			</div>

			<div class="input-group">
				<label for="amount">Amount</label>
				<div class="amount-input">
					<span class="currency">$</span>
					<input
						id="amount"
						name="amount"
						type="number"
						min="0"
						step="0.01"
						bind:value={formdata.amount}
						placeholder="0.00"
						required
					/>
				</div>
			</div>

			<div class="input-group">
				<label for="date">Date</label>
				<input id="date" name="date" type="date" bind:value={formdata.date} required />
			</div>

			<div class="input-group checkbox-group">
				<label class="toggle">
					<input name="processed" type="checkbox" bind:checked={formdata.processed} />
					<span class="slider"></span>
					<span class="label-text">Received</span>
				</label>
			</div>

			<div class="actions">
				<button type="submit" class="submit-btn" {disabled} class:loading={sending}>
					{#if !sending}
						<Icon icon="material-symbols:add-rounded" />
						<span>Add Income</span>
					{:else}
						<Icon icon="eos-icons:loading" />
						<span>Adding...</span>
					{/if}
				</button>
			</div>
		</div>
	</form>
</div>

<style>
	.add-income-container {
		background-color: #1e1e1e;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
		border: 1px solid rgba(255, 255, 255, 0.05);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1.5fr 1fr 1fr 1fr auto;
		gap: 1.25rem;
		align-items: flex-end;
	}

	@media (max-width: 1200px) {
		.form-grid {
			grid-template-columns: 1fr 1fr 1fr;
		}
		.checkbox-group {
			grid-column: span 1;
		}
		.actions {
			grid-column: span 3;
			justify-content: flex-end;
		}
		.submit-btn {
			width: 100%;
		}
	}

	@media (max-width: 768px) {
		.form-grid {
			grid-template-columns: 1fr 1fr;
		}
		.actions {
			grid-column: span 2;
		}
	}

	@media (max-width: 480px) {
		.form-grid {
			grid-template-columns: 1fr;
		}
		.actions {
			grid-column: span 1;
		}
	}

	.input-group { display: flex; flex-direction: column; gap: 0.5rem; }
	.input-group label { font-size: 0.85rem; color: var(--color-text-secondary); font-weight: 500; }

	input {
		background-color: #2a2a2a;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: white;
		padding: 0.75rem 1rem;
		font-size: 1rem;
		transition: all 0.2s;
		width: 100%;
	}

	input:focus {
		outline: none;
		border-color: var(--color-bg-primary);
		box-shadow: 0 0 0 2px rgba(29, 185, 84, 0.2);
	}

	.amount-input { position: relative; display: flex; align-items: center; }
	.currency { position: absolute; left: 1rem; color: var(--color-text-secondary); }
	.amount-input input { padding-left: 2rem; }
	.checkbox-group { justify-content: center; padding-bottom: 0.5rem; }

	.toggle { position: relative; display: flex; align-items: center; gap: 0.75rem; cursor: pointer; user-select: none; }
	.toggle input { position: absolute; opacity: 0; height: 0; width: 0; }
	.slider { position: relative; height: 24px; width: 44px; background-color: #444; transition: 0.4s; border-radius: 24px; }
	.slider:before { position: absolute; content: ''; height: 18px; width: 18px; left: 3px; bottom: 3px; background-color: white; transition: 0.4s; border-radius: 50%; }
	input:checked + .slider { background-color: var(--color-bg-primary); }
	input:checked + .slider:before { transform: translateX(20px); }
	.label-text { font-size: 0.9rem; color: white; }

	.submit-btn {
		background-color: var(--color-bg-primary);
		color: black;
		border: none;
		border-radius: 8px;
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.submit-btn:hover:not(:disabled) { background-color: var(--color-bg-hover); transform: translateY(-1px); }
	.submit-btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
