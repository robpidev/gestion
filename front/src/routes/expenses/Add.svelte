<script>
	import Icon from '@iconify/svelte';
</script>

<form action="">
	<input type="text" placeholder="Concept" />
	<input type="number" min="0" placeholder="Amount" />
	<button type="submit">
		<Icon icon="streamline:add-1-solid" />
	</button>
</form>

<style>
	form {
		width: 100%;
		border: solid 1px rgba(255, 255, 255, 0.1);
		padding: 1rem;
		display: flex;
		gap: 1rem;
		border-radius: 6px;
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
</style>
