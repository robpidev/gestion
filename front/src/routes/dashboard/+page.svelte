<script lang="ts">
	import { userState } from '$lib/store/user.svelte';
	import Icon from '@iconify/svelte';
	import type { Expense } from '$lib/interfaces/expenses';
	import type { Income } from '$lib/interfaces/incomes';

	let { data } = $props();

	// Calculate Stats
	const now = new Date();
	const currentMonth = now.getMonth();
	const currentYear = now.getFullYear();

	const monthlyExpenses = $derived(
		data.expenses
			?.filter((e: Expense) => {
				const d = new Date(e.date);
				return d.getMonth() === currentMonth && d.getFullYear() === currentYear;
			})
			.reduce((acc: number, e: Expense) => acc + e.amount, 0) || 0
	);

	const monthlyIncome = $derived(
		data.incomes
			?.filter((i: Income) => {
				const d = new Date(i.date);
				return d.getMonth() === currentMonth && d.getFullYear() === currentYear;
			})
			.reduce((acc: number, i: Income) => acc + i.amount, 0) || 0
	);

	const totalIncome = $derived(data.incomes?.reduce((acc: number, i: Income) => acc + i.amount, 0) || 0);
	const totalExpenses = $derived(data.expenses?.reduce((acc: number, e: Expense) => acc + e.amount, 0) || 0);
	const currentBalance = $derived(totalIncome - totalExpenses);

	const savingsRate = $derived(monthlyIncome > 0 ? ((monthlyIncome - monthlyExpenses) / monthlyIncome) * 100 : 0);

	const recentActivity = $derived.by(() => {
		const combined = [
			...(data.expenses?.map((e: Expense) => ({ ...e, type: 'expense' })) || []),
			...(data.incomes?.map((i: Income) => ({ ...i, type: 'income' })) || [])
		];
		return combined
			.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
			.slice(0, 5);
	});

	const stats = $derived([
		{ 
			label: 'Monthly Expenses', 
			value: `$${monthlyExpenses.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`, 
			icon: 'material-symbols:trending-down-rounded', 
			color: '#ff4d4d' 
		},
		{ 
			label: 'Monthly Income', 
			value: `$${monthlyIncome.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`, 
			icon: 'material-symbols:trending-up-rounded', 
			color: 'var(--color-bg-primary)' 
		},
		{ 
			label: 'Current Balance', 
			value: `$${currentBalance.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`, 
			icon: 'material-symbols:account-balance-wallet-outline', 
			color: '#4d94ff' 
		},
		{ 
			label: 'Savings Rate', 
			value: `${savingsRate.toFixed(1)}%`, 
			icon: 'material-symbols:savings-outline', 
			color: '#ffaa00' 
		}
	]);

	function formatDate(dateStr: string) {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric'
		}).format(new Date(dateStr));
	}
</script>

<div class="welcome-section">
	<h1>Welcome back, {userState.user?.name}! 👋</h1>
	<p>Here's what's happening with your finances this month.</p>
</div>

<div class="stats-grid">
	{#each stats as stat}
		<div class="stat-card">
			<div class="stat-icon" style="background-color: {stat.color}20; color: {stat.color}">
				<Icon icon={stat.icon} />
			</div>
			<div class="stat-details">
				<span class="stat-label">{stat.label}</span>
				<span class="stat-value">{stat.value}</span>
			</div>
		</div>
	{/each}
</div>

<div class="recent-activity">
	<h2>Recent Activity</h2>
	{#if recentActivity.length > 0}
		<div class="activity-list">
			{#each recentActivity as item}
				<div class="activity-item">
					<div class="activity-icon" class:income={item.type === 'income'}>
						<Icon icon={item.type === 'income' ? 'material-symbols:add-circle-outline' : 'material-symbols:remove-circle-outline'} />
					</div>
					<div class="activity-info">
						<span class="activity-desc">{item.description}</span>
						<span class="activity-date">{formatDate(item.date)}</span>
					</div>
					<div class="activity-amount" class:income={item.type === 'income'}>
						{item.type === 'income' ? '+' : '-'}${item.amount.toFixed(2)}
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="empty-activity">
			<Icon icon="material-symbols:history-rounded" />
			<p>No recent activity to show.</p>
			<a href="/dashboard/expenses" class="action-link">Add your first expense</a>
		</div>
	{/if}
</div>

<style>
	.welcome-section {
		margin-bottom: 2.5rem;
	}

	.welcome-section h1 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}

	.welcome-section p {
		color: var(--color-text-secondary);
		font-size: 1.1rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	.stat-card {
		background-color: #1e1e1e;
		padding: 1.5rem;
		border-radius: 16px;
		display: flex;
		align-items: center;
		gap: 1.25rem;
		border: 1px solid rgba(255, 255, 255, 0.05);
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.stat-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
	}

	.stat-icon {
		width: 56px;
		height: 56px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.75rem;
	}

	.stat-details {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.9rem;
		color: var(--color-text-secondary);
		font-weight: 500;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
	}

	.recent-activity h2 {
		font-size: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		background-color: #1e1e1e;
		border-radius: 16px;
		padding: 1rem;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.activity-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		border-radius: 12px;
		transition: background-color 0.2s;
	}

	.activity-item:hover {
		background-color: rgba(255, 255, 255, 0.03);
	}

	.activity-icon {
		font-size: 1.5rem;
		color: #ff4d4d;
		display: flex;
	}

	.activity-icon.income {
		color: var(--color-bg-primary);
	}

	.activity-info {
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.activity-desc {
		font-weight: 500;
		color: white;
	}

	.activity-date {
		font-size: 0.8rem;
		color: var(--color-text-secondary);
	}

	.activity-amount {
		font-weight: 700;
		color: #ff4d4d;
	}

	.activity-amount.income {
		color: var(--color-bg-primary);
	}

	.empty-activity {
		background-color: #1e1e1e;
		border-radius: 16px;
		padding: 4rem 2rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		border: 1px dashed rgba(255, 255, 255, 0.1);
		color: var(--color-text-secondary);
	}

	.empty-activity :global(svg) {
		font-size: 3rem;
		opacity: 0.2;
	}

	.action-link {
		margin-top: 0.5rem;
		color: var(--color-bg-primary);
		text-decoration: none;
		font-weight: 600;
		transition: opacity 0.2s;
	}

	.action-link:hover {
		text-decoration: underline;
	}
</style>
