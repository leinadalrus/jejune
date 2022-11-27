<script>
	export let item
	export let returnTo

	$: url = !item.domain 
    ? process.env.VITE_SUPABASE_PUB_URL+`/${item.url}` 
    : process.env.VITE_SUPABASE_PUB_URL+`/${item.id}`
</script>

<a href={returnTo}>Go Back</a>

<article>
	<a href='{url}'>
		<h1>{item.title}</h1>
		{#if item.domain}
			<small>{item.domain}</small>
		{/if}
	</a>

	<p class='meta'>submitted by {item.user} {item.timeAgo}
</article>

<style>
	article {
		margin: 0 0 1em 0
	}

	a {
		display: block;
		margin: 0 0 1em 0
	}

	h1 {
		font-size: 1.4em;
		margin: 0
	}
</style>