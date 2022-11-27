<script>
	import { beforeUpdate } from 'svelte'
	import './DashboardModel.svelte'
    import DashboardModel from './DashboardModel.svelte';

	const PAGE_SIZE = 20

	export let page

	let items
	let offset

	$: fetch(process.env.VITE_SUPABASE_PUB_URL+`/{user}/{docs}?page=${id}`)
		.then(r => r.json())
		.then(data => {
			items = data
			offset = PAGE_SIZE * (page - 1)
			window.scrollTo(0, 0)
		})
</script>

{#if items}
	{#each items as item, i}
		<DashboardModel {item} {i} {offset}/>
	{/each}

	<a href='#/top/{page + 1}'>page {page + 1}</a>
{:else}
	<p class='loading'>loading...</p>
{/if}

<style>
	a {
		padding: 2em;
		display: block
	}

	.loading {
		opacity: 0;
		animation: 0.4s 0.8s forwards fade-in
	}

	@keyframes fade-in {
		from { opacity: 0 }
		to { opacity: 1 }
	}
</style>