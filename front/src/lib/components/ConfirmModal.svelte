<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade, scale } from 'svelte/transition';

	let { 
		show = false, 
		title = 'Are you sure?', 
		message = 'This action cannot be undone.', 
		confirmText = 'Delete', 
		cancelText = 'Cancel',
		onConfirm, 
		onCancel 
	} = $props();

	function handleConfirm() {
		onConfirm();
	}

	function handleCancel() {
		onCancel();
	}
</script>

{#if show}
	<!-- Backdrop -->
	<div 
		class="modal-backdrop" 
		transition:fade={{ duration: 200 }}
		onclick={handleCancel}
		onkeydown={(e) => e.key === 'Escape' && handleCancel()}
		role="button"
		tabindex="0"
	>
		<!-- Modal Card -->
		<div 
			class="modal-card" 
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="none"
		>
			<div class="modal-icon">
				<Icon icon="material-symbols:warning-outline-rounded" />
			</div>
			
			<h2>{title}</h2>
			<p>{message}</p>
			
			<div class="modal-actions">
				<button class="btn-cancel" onclick={handleCancel}>
					{cancelText}
				</button>
				<button class="btn-confirm" onclick={handleConfirm}>
					{confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1.5rem;
	}

	.modal-card {
		background-color: #1e1e1e;
		width: 100%;
		max-width: 400px;
		border-radius: 20px;
		padding: 2rem;
		text-align: center;
		border: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
	}

	.modal-icon {
		width: 64px;
		height: 64px;
		background-color: rgba(255, 77, 77, 0.1);
		color: #ff4d4d;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1.5rem;
		font-size: 2.5rem;
	}

	h2 {
		margin-bottom: 0.75rem;
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	p {
		color: var(--color-text-secondary);
		margin-bottom: 2rem;
		line-height: 1.5;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
	}

	button {
		flex: 1;
		padding: 0.8rem;
		border-radius: 12px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-cancel {
		background-color: #333;
		color: white;
	}

	.btn-cancel:hover {
		background-color: #444;
	}

	.btn-confirm {
		background-color: #ff4d4d;
		color: white;
	}

	.btn-confirm:hover {
		background-color: #ff3333;
		transform: translateY(-1px);
	}

	.btn-confirm:active {
		transform: translateY(0);
	}
</style>
