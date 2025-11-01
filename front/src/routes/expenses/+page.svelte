<script>
	import Add from './Add.svelte';
	import Expenses from './Expenses.svelte';

	let concepts = [
		{
			id: 1,
			description: 'Concepto 1',
			amount: 100,
			date: new Date('2023-01-01')
		},
		{
			id: 2,
			description: 'Concepto 2',
			amount: 200,
			date: new Date('2023-01-01')
		},
		{
			id: 3,
			description: 'Concepto 3',
			amount: 300,
			date: new Date('2023-01-01')
		},
		{
			id: 5,
			description: 'Concepto 3',
			amount: 300,
			date: new Date('2022-01-01')
		},
		{
			id: 5,
			description: 'Concepto 3',
			amount: 300,
			date: new Date('2022-01-01')
		}
	];
</script>

<Add />
<Expenses {concepts} />
