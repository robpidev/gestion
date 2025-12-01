<script lang="ts">
	import { enhance } from '$app/forms';
	let { form } = $props();

	let name = $state('');
	let valid_name = $state(true);

	let lastname = $state('');
	let valid_lastname = $state(true);

	let username = $state('');
	let valid_username = $state(true);

	let password = $state('');
	let valid_password = $state(true);

	let password2 = $state('');
	let valid_password2 = $state(true);

	$effect(() => {
		valid_name = validate(name);
	});

	$effect(() => {
		valid_lastname = validate(lastname);
	});

	$effect(() => {
		valid_username = validate(username);
	});

	$effect(() => {
		valid_password = validate(password);
	});

	$effect(() => {
		valid_password2 = password == password2;
	});

	function validate(value: string) {
		let regex = /^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]{0,50}$/;
		return regex.test(value) ? true : false;
	}

	let disabled = $state(false);
</script>

<div>
	<span class="title">Signup</span>
	<form
		action="?/create"
		method="POST"
		use:enhance={() => {
			disabled = true;
			return async ({ update }) => {
				await update();
				disabled = false;
			};
		}}
	>
		<label
			>name:
			<input type="text" bind:value={name} name="name" placeholder="Insert your name" required />
			{#if !valid_name}
				<span class="field-error">Invalid name</span>
			{/if}
		</label>
		<label>
			lastname:
			<input
				type="text"
				bind:value={lastname}
				name="lastname"
				placeholder="Insert your lastname"
				required
			/>
			{#if !valid_lastname}
				<span class="field-error">Invalid lastname</span>
			{/if}
		</label>
		<label>
			username:
			<input
				type="text"
				bind:value={username}
				name="username"
				placeholder="Insert your username"
				required
			/>
			{#if !valid_username}
				<span class="field-error">Invalid username</span>
			{/if}
		</label>
		<label>
			password:
			<input
				type="text"
				bind:value={password}
				name="password"
				placeholder="Insert your password"
				required
			/>
			{#if !valid_password}
				<span class="field-error">Invalid password</span>
			{/if}
		</label>
		<label>
			Password:
			<input
				type="text"
				bind:value={password2}
				name="password"
				placeholder="Comfirm your password"
				required
			/>
			{#if !valid_password2}
				<span class="field-error">Password do not match</span>
			{/if}
		</label>

		<button
			disabled={!valid_name ||
				!valid_lastname ||
				!valid_username ||
				!valid_password ||
				!valid_password2 ||
				disabled}>Submit</button
		>
	</form>
	<p class="error">
		{#if form?.error}
			{form.error}
		{/if}
	</p>
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
