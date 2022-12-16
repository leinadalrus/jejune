<script lang='js'>
import LeftSidebar from '../components/LeftSidebar/LeftSidebar.svelte'

$: IsShowingLeftSidebar = false
</script>

<template>
  <input type='button' on:click={() => (
    IsShowingLeftSidebar = true
  )}>
  {#if IsShowingLeftSidebar}
    <div>
      <input type='button' on:click={() => (
        IsShowingLeftSidebar = false
      )}>
    </div>
  {/if}
</template>