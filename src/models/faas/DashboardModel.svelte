<svelte:options immutable />

<script lang='ts'>
  import { afterUpdate } from 'svelte'
  import fade from './AnimateNiches.svelte'
  import forwards from './AnimateNiches.svelte'
  import fallback from './AnimateNiches.svelte'

  export let DashboardModel: any
  let div: any

  afterUpdate(() => {
    new fade(div)
  })
</script>

<div bind:this={div} on:click on:keypress>
  { DashboardModel.done ? forwards || fallback : undefined }
  { DashboardModel.body }
</div>

<style>
  div {
    cursor: pointer;
    line-height: 1.2;
  }
</style>