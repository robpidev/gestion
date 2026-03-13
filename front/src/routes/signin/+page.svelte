<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { type User } from '$lib/interfaces/user.js';
	import { userState } from '$lib/store/user.svelte';
	import Icon from '@iconify/svelte';
	import favicon from '$lib/assets/favicon.svg';

	let { form } = $props();
	let loading = $state(false);
</script>

<svelte:head>
	<title>Login | Gestion</title>
</svelte:head>

<div class="auth-page">
	<div class="auth-card">
		<div class="auth-header">
			<div class="logo">
				<img src={favicon} alt="Logo" />
				<span>Gestion</span>
			</div>
			<h1>Welcome Back</h1>
			<p>Enter your credentials to access your account</p>
		</div>

		<form
			action="?/login"
			method="POST"
			use:enhance={() => {
				loading = true;
				return async ({ update, result }) => {
					if (result.type === 'success') {
						userState.user = result.data?.user as User;
						await goto('/dashboard');
					}
					loading = false;
					await update();
				};
			}}
		>
			<div class="input-group">
				<label for="username">Username</label>
				<div class="input-wrapper">
					<Icon icon="material-symbols:person-outline-rounded" />
					<input id="username" type="text" name="username" placeholder="your_username" required />
				</div>
			</div>

			<div class="input-group">
				<label for="password">Password</label>
				<div class="input-wrapper">
					<Icon icon="material-symbols:lock-outline-rounded" />
					<input id="password" type="password" name="password" placeholder="••••••••" required />
				</div>
			</div>

			{#if form?.error}
				<div class="error-message">
					<Icon icon="material-symbols:error-outline-rounded" />
					<span>{form.error}</span>
				</div>
			{/if}

			<button disabled={loading} type="submit" class="submit-btn">
				{#if loading}
					<Icon icon="eos-icons:loading" />
					<span>Logging in...</span>
				{:else}
					<span>Login</span>
				{/if}
			</button>
		</form>

		<div class="auth-footer">
			<p>Don't have an account? <a href="/signup">Sign up</a></p>
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
		max-width: 420px;
		background-color: #1e1e1e;
		padding: 2.5rem;
		border-radius: 20px;
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
		margin-bottom: 1.5rem;
	}

	.logo img {
		width: 40px;
		height: 40px;
	}

	.logo span {
		font-size: 1.5rem;
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

	.input-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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
		font-size: 1.25rem;
		color: var(--color-text-secondary);
	}

	input {
		width: 100%;
		background-color: #2a2a2a;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: white;
		padding: 0.75rem 1rem 0.75rem 3rem;
		font-size: 1rem;
		transition: all 0.2s;
	}

	input:focus {
		outline: none;
		border-color: var(--color-bg-primary);
		background-color: #333;
		box-shadow: 0 0 0 3px rgba(29, 185, 84, 0.1);
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #ff4d4d;
		font-size: 0.85rem;
		background-color: rgba(255, 77, 77, 0.1);
		padding: 0.75rem;
		border-radius: 8px;
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
		opacity: 0.7;
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
