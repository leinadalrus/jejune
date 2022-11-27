<script lang='ts'>
	import { onMount } from 'svelte'
	import 'ForwardList.svelte'
	import Enumerables from './Enumerables.svelte'
	import type item from './Enumerables.svelte'
	import type prevInto from './Enumerables.svelte'

	let items: Array<item>
  let prevIntos: Array<prevInto>
	let page: any

	async function hashchange () {
		// the poor man's router!
		const path = window.location.hash.slice(1)

		if (path.startsWith('/item')) {
			const id = path.slice(6)
			items = await fetch(process.env.VITE_SUPABASE_PUB_URL+`/item/${id}`).then(r => r.json())

			window.scrollTo(0,0)
		} else if (path.startsWith('/top')) {
			page = +path.slice(5)
			items
		} else {
			window.location.hash = '/top/1'
		}
	}

	onMount(hashchange)
</script>

<svelte:window on:hashchange={hashchange}/>

<main>
	{#if items}
		<prevInto={page}/>
	{:else if page}
		<items />
	{/if}
</main>

<style>
	main {
		position: relative;
		max-width: 800px;
		margin: 0 auto;
		min-height: 101vh;
		padding: 1em
	}

	main :global(.meta) {
		color: #999;
		font-size: 12px;
		margin: 0 0 1em 0
	}

	main :global(a) {
		color: rgb(0,0,150)
	}
</style>