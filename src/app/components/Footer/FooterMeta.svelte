<script lang='ts'>

import StatusBanner from './StatusBanner.svelte'


export let path: string
</script>

<footer>
  <StatusBanner bind:value={path} />
</footer>