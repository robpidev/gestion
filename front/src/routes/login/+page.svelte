<script lang="ts">
	import { enhance } from '$app/forms';

	let { form } = $props();
	let disabled = false;
</script>

<div>
	<span class="title">Login</span>
	<form
		action="?/login"
		method="POST"
		use:enhance={() => {
			disabled = true;
			return async ({ update }) => {
				await update();
				disabled = false;
			};
		}}
	>
		<label>
			username:
			<input type="text" name="username" placeholder="insert username" required />
		</label>
		<label>
			password:
			<input type="password" name="password" placeholder="inserte password" required />
		</label>
		<button type="submit">Login</button>
	</form>
	{#if form?.error}
		<p class="error">{form.error}</p>
	{/if}
</div>

<style>
	div {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		padding: 0.5rem 1rem;
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-self: center;
		align-items: center;
		padding: 1rem;
		gap: 1rem;
	}

	label {
		/* background: rgba(255, 255, 255, 0.1); */
		position: relative;
	}

	.field-error {
		position: absolute;
		right: 0;
		bottom: -1.2rem;
		width: max-content;
		color: red;
	}
	.title {
		font-size: 2rem;
		font-weight: bold;
		/* background: cyan; */
		width: min-content;
	}

	button {
		color: black;
		border-radius: 6px;
		border: solid 1px rgba(255, 255, 255, 0.1);
		padding: 0.5rem 1rem;
		margin-top: 1rem;
	}

	/* button[disabled='true'] { */
	/* 	opacity: 0.5; */
	/* } */

	button:hover {
		background: var(--color-border-hover);
	}

	button:disabled {
		opacity: 0.5;
	}

	button:active {
		border: solid 2px var(--color-border-active);
		background: var(--color-border-active);
	}

	input {
		font-size: 1.2rem;
		color: black;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		width: 20rem;
		background: rgba(255, 255, 255, 0.2);
		border: solid 1px rgba(255, 255, 255, 0.1);
		color: white;
		outline: none;
		/* border: solid 1px var(--color-border-primary); */
	}

	input:hover {
		border: solid 1px var(--color-border-hover);
	}

	input:focus {
		border: solid 1px var(--color-border-primary);
		background: rgba(255, 255, 255, 0.3);
	}

	label {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	form {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: min-content;
		padding: 1rem;
		border-radius: 8px;
	}

	.error {
		color: red;
	}
</style>
