<svelte:options immutable />

<script>
  import { afterUpdate } from 'svelte'
  import { fade, next, previous, unknown } from './animateNiches.js';

  export let DashboardModel
  let div

  afterUpdate(() => {
    fade(div)
  })
</script>

<div bind:this={div} on:click on:keypress>
  { DashboardModel.done ? next() || previous() : unknown() }
  { DashboardModel.body }
</div>

<style>
  div {
    cursor: pointer;
    line-height: 1.2;
  }
</style>