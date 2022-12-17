<script lang="ts">
  import '../app.css'

  import type { LayoutData } from './$types'
  export let data: LayoutData
</script>


<div class="submenu">
  <h1>Settings</h1>
  {#each data.sections as section}
    <a href="/settings/{section.slug}">{section.title}</a>
  {/each}
</div>