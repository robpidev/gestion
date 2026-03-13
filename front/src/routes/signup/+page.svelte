<script lang="ts">
	import { enhance } from '$app/forms';
	import Icon from '@iconify/svelte';
	import favicon from '$lib/assets/favicon.svg';

	let { form } = $props();

	let name = $state('');
	let lastname = $state('');
	let username = $state('');
	let password = $state('');
	let password2 = $state('');

	let loading = $state(false);

	let valid_name = $derived(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]{2,50}$/.test(name));
	let valid_lastname = $derived(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]{2,50}$/.test(lastname));
	let valid_username = $derived(/^[\.a-zA-Z0-9_-]{5,30}$/.test(username));
	let valid_password = $derived(password.length >= 8);
	let valid_password2 = $derived(password === password2 && password2.length > 0);

	let canSubmit = $derived(
		valid_name && valid_lastname && valid_username && valid_password && valid_password2 && !loading
	);
</script>

<svelte:head>
	<title>Sign Up | Gestion</title>
</svelte:head>

<div class="auth-page">
	<div class="auth-card">
		<div class="auth-header">
			<div class="logo">
				<img src={favicon} alt="Logo" />
				<span>Gestion</span>
			</div>
			<h1>Create Account</h1>
			<p>Join us to start managing your finances</p>
		</div>

		<form
			action="?/create"
			method="POST"
			use:enhance={() => {
				loading = true;
				return async ({ update }) => {
					loading = false;
					await update();
				};
			}}
		>
			<div class="form-row">
				<div class="input-group">
					<label for="name">First Name</label>
					<input
						id="name"
						type="text"
						bind:value={name}
						name="name"
						placeholder="John"
						required
						class:invalid={name && !valid_name}
					/>
				</div>
				<div class="input-group">
					<label for="lastname">Last Name</label>
					<input
						id="lastname"
						type="text"
						bind:value={lastname}
						name="lastname"
						placeholder="Doe"
						required
						class:invalid={lastname && !valid_lastname}
					/>
				</div>
			</div>

			<div class="input-group">
				<label for="username">Username</label>
				<div class="input-wrapper">
					<Icon icon="material-symbols:person-outline-rounded" />
					<input
						id="username"
						type="text"
						bind:value={username}
						name="username"
						placeholder="johndoe123"
						required
						class:invalid={username && !valid_username}
					/>
				</div>
				{#if username && !valid_username}
					<span class="field-error">5-30 characters, letters, numbers, dots, underscores</span>
				{/if}
			</div>

			<div class="input-group">
				<label for="password">Password</label>
				<div class="input-wrapper">
					<Icon icon="material-symbols:lock-outline-rounded" />
					<input
						id="password"
						type="password"
						bind:value={password}
						name="password"
						placeholder="••••••••"
						required
						class:invalid={password && !valid_password}
					/>
				</div>
				{#if password && !valid_password}
					<span class="field-error">At least 8 characters</span>
				{/if}
			</div>

			<div class="input-group">
				<label for="passwordConfirm">Confirm Password</label>
				<div class="input-wrapper">
					<Icon icon="material-symbols:lock-reset-outline-rounded" />
					<input
						id="passwordConfirm"
						type="password"
						bind:value={password2}
						placeholder="••••••••"
						required
						class:invalid={password2 && !valid_password2}
					/>
				</div>
				{#if password2 && !valid_password2}
					<span class="field-error">Passwords do not match</span>
				{/if}
			</div>

			{#if form?.error}
				<div class="error-message">
					<Icon icon="material-symbols:error-outline-rounded" />
					<span>{form.error}</span>
				</div>
			{/if}

			<button disabled={!canSubmit} type="submit" class="submit-btn">
				{#if loading}
					<Icon icon="eos-icons:loading" />
					<span>Creating account...</span>
				{:else}
					<span>Sign Up</span>
				{/if}
			</button>
		</form>

		<div class="auth-footer">
			<p>Already have an account? <a href="/signin">Login</a></p>
		</div>
	</div>
</div>

<style>
	.auth-page {
		min-height: 100vh;
		width: 100vw;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: #121212;
		padding: 1.5rem;
	}

	.auth-card {
		width: 100%;
		max-width: 480px;
		background-color: #1e1e1e;
		padding: 2.5rem;
		border-radius: 24px;
		border: 1px solid rgba(255, 255, 255, 0.05);
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
	}

	.auth-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.logo {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 1.25rem;
	}

	.logo img {
		width: 36px;
		height: 36px;
	}

	.logo span {
		font-size: 1.4rem;
		font-weight: 700;
		color: var(--color-bg-primary);
	}

	.auth-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		color: white;
	}

	.auth-header p {
		color: var(--color-text-secondary);
		font-size: 0.9rem;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	@media (max-width: 480px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}

	.input-group {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.input-group label {
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--color-text-secondary);
	}

	.input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.input-wrapper :global(svg) {
		position: absolute;
		left: 1rem;
		font-size: 1.2rem;
		color: var(--color-text-secondary);
	}

	input {
		width: 100%;
		background-color: #2a2a2a;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: white;
		padding: 0.7rem 1rem;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input-wrapper input {
		padding-left: 2.8rem;
	}

	input:focus {
		outline: none;
		border-color: var(--color-bg-primary);
		background-color: #333;
		box-shadow: 0 0 0 3px rgba(29, 185, 84, 0.1);
	}

	input.invalid {
		border-color: #ff4d4d;
	}

	.field-error {
		font-size: 0.75rem;
		color: #ff4d4d;
		margin-top: 0.1rem;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #ff4d4d;
		font-size: 0.85rem;
		background-color: rgba(255, 77, 77, 0.1);
		padding: 0.75rem;
		border-radius: 10px;
	}

	.submit-btn {
		background-color: var(--color-bg-primary);
		color: black;
		border: none;
		border-radius: 12px;
		padding: 0.9rem;
		font-size: 1rem;
		font-weight: 700;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		transition: all 0.2s;
		margin-top: 0.5rem;
	}

	.submit-btn:hover:not(:disabled) {
		background-color: var(--color-bg-hover);
		transform: translateY(-1px);
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.auth-footer {
		margin-top: 1.5rem;
		text-align: center;
		font-size: 0.9rem;
		color: var(--color-text-secondary);
	}

	.auth-footer a {
		color: var(--color-bg-primary);
		text-decoration: none;
		font-weight: 600;
	}

	.auth-footer a:hover {
		text-decoration: underline;
	}
</style>
