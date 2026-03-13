<script lang="ts">
	import Icon from '@iconify/svelte';

	import { enhance } from '$app/forms';

	interface FormData {
		processed: boolean;
		description: string;
		amount: number;
		date: string;
	}
	let formdata = $state({} as FormData);
	formdata.date = new Date().toISOString().split('T')[0];
	formdata.processed = false;

	let disabled: boolean = $derived.by(
		() => !formdata.amount || !formdata.description || !formdata.date || !formdata.amount
	);

	$inspect(disabled).with(console.log);
	let sending = $state(false);
</script>

<form
	method="POST"
	action="?/create"
	use:enhance={() => {
		sending = true;
		return async ({ update }) => {
			sending = false;
			await update();
		};
	}}
>
	<label>
		<input name="processed" type="checkbox" bind:checked={formdata.processed} />
		<span>Processed</span>
	</label>
	<input
		bind:value={formdata.description}
		name="description"
		type="text"
		placeholder="Concept"
		required
	/>
	<input
		name="amount"
		type="number"
		min="0"
		bind:value={formdata.amount}
		placeholder="Amount"
		required
	/>
	<input name="date" type="date" bind:value={formdata.date} required />
	{#if !sending}
		<button type="submit" {disabled}>
			<Icon icon="streamline:add-1-solid" />
		</button>
	{:else}
		<Icon icon="eos-icons:loading" />
	{/if}
	<!-- else if content here -->
</form>

<style>
	form {
		width: 100%;
		border: solid 1px rgba(255, 255, 255, 0.1);
		padding: 1rem;
		display: flex;
		gap: 1rem;
		border-radius: 6px;
		align-items: center;
		margin: 1rem 0;
		display: flex;
		flex-wrap: wrap;
	}

	input {
		flex: 1;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		border: solid 1px rgba(255, 255, 255, 0.1);
		outline: none;
		padding: 0.5rem 1rem;
		font-family: 1.5rem;
	}

	input:hover {
		border: solid 1px var(--color-border-hover);
	}

	input:focus {
		border: solid 1px var(--color-border-focus);
	}

	button {
		background: none;
		border: solid 1px rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		cursor: pointer;
		font-size: 2rem;
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

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
