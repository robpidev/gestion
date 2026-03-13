<script lang="ts">
	import { goto } from '$app/navigation';
	import favicon from '$lib/assets/favicon.svg';
	import { userState } from '$lib/store/user.svelte';
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import { page } from '$app/state';

	let { data, children } = $props();
	let isSidebarOpen = $state(false);

	onMount(() => {
		userState.user = data.user;
	});

	const navItems = [
		{ name: 'Dashboard', href: '/dashboard', icon: 'material-symbols:dashboard-outline' },
		{ name: 'Expenses', href: '/dashboard/expenses', icon: 'material-symbols:payments-outline' },
		{ name: 'Incomes', href: '/dashboard/incomes', icon: 'material-symbols:account-balance-wallet-outline' }
	];

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	// Close sidebar when navigating on mobile
	$effect(() => {
		if (page.url.pathname) {
			isSidebarOpen = false;
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="dashboard-container">
	<!-- Mobile Header -->
	<header class="mobile-header">
		<button class="menu-toggle" onclick={toggleSidebar} aria-label="Toggle Menu">
			<Icon icon={isSidebarOpen ? 'material-symbols:close-rounded' : 'material-symbols:menu-rounded'} />
		</button>
		<div class="logo">
			<img src={favicon} alt="Logo" />
			<span>Gestion</span>
		</div>
		<div class="mobile-user">
			{userState.user?.name?.charAt(0) || 'U'}
		</div>
	</header>

	<!-- Sidebar / Navigation -->
	<aside class="sidebar" class:open={isSidebarOpen}>
		<div class="sidebar-header desktop-only">
			<div class="logo">
				<img src={favicon} alt="Logo" />
				<span>Gestion</span>
			</div>
		</div>

		<nav class="nav-menu">
			<ul>
				{#each navItems as item}
					<li>
						<a href={item.href} class:active={page.url.pathname === item.href}>
							<Icon icon={item.icon} />
							<span>{item.name}</span>
						</a>
					</li>
				{/each}
			</ul>
		</nav>

		<div class="user-section">
			<div class="user-info">
				<div class="avatar">
					{userState.user?.name?.charAt(0) || 'U'}
				</div>
				<div class="details">
					<span class="name">{userState.user?.name}</span>
					<span class="role">User</span>
				</div>
			</div>
			<a href="/signin" class="logout-btn">
				<Icon icon="material-symbols:logout-rounded" />
				<span>Logout</span>
			</a>
		</div>
	</aside>

	<!-- Overlay for mobile -->
	{#if isSidebarOpen}
		<div class="sidebar-overlay" onclick={toggleSidebar} onkeydown={(e) => e.key === 'Enter' && toggleSidebar()} role="button" tabindex="0"></div>
	{/if}

	<main class="main-content">
		<header class="content-header desktop-only">
			<h1>{navItems.find((i) => i.href === page.url.pathname)?.name || 'Dashboard'}</h1>
		</header>
		<div class="content-body">
			{@render children?.()}
		</div>
	</main>
</div>

<style>
	:global(body) {
		overflow: hidden; /* Prevent body scroll, use container scroll */
		height: 100vh;
		width: 100vw;
	}

	.dashboard-container {
		display: flex;
		width: 100%;
		height: 100vh;
		background-color: #121212;
		color: white;
		position: relative;
	}

	/* Mobile Header Styles */
	.mobile-header {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 64px;
		background-color: #000;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		z-index: 50;
		padding: 0 1rem;
		align-items: center;
		justify-content: space-between;
	}

	.menu-toggle {
		background: none;
		border: none;
		color: white;
		font-size: 1.75rem;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
	}

	.mobile-user {
		width: 32px;
		height: 32px;
		background-color: var(--color-bg-primary);
		color: black;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: bold;
		font-size: 0.8rem;
	}

	.sidebar {
		width: 260px;
		background-color: #000;
		display: flex;
		flex-direction: column;
		padding: 1.5rem;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		z-index: 100;
		transition: transform 0.3s ease;
	}

	.sidebar-header {
		margin-bottom: 2.5rem;
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.5rem;
		font-weight: bold;
		color: var(--color-bg-primary);
	}

	.logo img {
		width: 32px;
		height: 32px;
	}

	.nav-menu {
		flex: 1;
	}

	.nav-menu ul {
		list-style: none;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.nav-menu a {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.8rem 1rem;
		text-decoration: none;
		color: var(--color-text-secondary);
		border-radius: 8px;
		transition: all 0.2s;
		font-size: 1rem;
	}

	.nav-menu a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: white;
	}

	.nav-menu a.active {
		background-color: var(--color-bg-primary);
		color: white;
	}

	.user-section {
		margin-top: auto;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.avatar {
		width: 40px;
		height: 40px;
		background-color: var(--color-bg-primary);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: bold;
		color: black;
	}

	.details {
		display: flex;
		flex-direction: column;
	}

	.name {
		font-weight: 600;
		font-size: 0.9rem;
	}

	.role {
		font-size: 0.75rem;
		color: var(--color-text-secondary);
	}

	.logout-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #ff4d4d;
		text-decoration: none;
		font-size: 0.9rem;
		padding: 0.5rem;
		border-radius: 6px;
		transition: background-color 0.2s;
	}

	.logout-btn:hover {
		background-color: rgba(255, 77, 77, 0.1);
	}

	.sidebar-overlay {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 80;
	}

	.main-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow-y: auto;
		overflow-x: hidden;
		-webkit-overflow-scrolling: touch;
	}

	.content-header {
		padding: 1.5rem 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		background-color: rgba(18, 18, 18, 0.8);
		backdrop-filter: blur(10px);
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.content-header h1 {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.content-body {
		padding: 2rem;
		max-width: 1200px;
		width: 100%;
		margin: 0 auto;
		flex: 1;
	}

	/* Desktop only vs Mobile only utilities */
	@media (max-width: 768px) {
		.mobile-header {
			display: flex;
		}

		.desktop-only {
			display: none;
		}

		.sidebar {
			position: fixed;
			left: 0;
			top: 0;
			bottom: 0;
			transform: translateX(-100%);
		}

		.sidebar.open {
			transform: translateX(0);
		}

		.sidebar-overlay {
			display: block;
		}

		.main-content {
			padding-top: 64px;
		}

		.content-body {
			padding: 1.5rem 1rem;
		}
	}
</style>
