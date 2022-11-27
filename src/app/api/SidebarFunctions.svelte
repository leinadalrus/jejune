<script>
import { LeftSidebar } from './pages/components/LeftSidebar/LeftSidebar.astro'

$: IsShowingLeftSidebar = false

export class MenuToggle {
  get sidebarShows() {
    return this._sidebar
  }

  set sidebarShows(value) {
    this._sidebar = value
  }

  #sidebarShowing = <LeftSidebar bind:this={IsShowingLeftSidebar} />
  #setSidebarShown = <LeftSidebar bind:this={IsShowingLeftSidebar} />
  #_sidebar = [this.#sidebarShowing, this.#setSidebarShown]
}
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