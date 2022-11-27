<script lang='ts'>
import getLanguageFromURL from '../Sidebar/Sidebar.svelte'
import SIDEBAR  from '../Sidebar/Sidebar.svelte'

const publicURL = process.env.VITE_SUPABASE_PUBLIC_URL
const matchParams = publicURL.endsWith('/') 
  ? publicURL.slice(1, -1) 
  : publicURL.slice(1)

const langCode = getLanguageFromURL(matchParams)
const sidebar = SIDEBAR[langCode]
export const LeftSidebar: any = {svelteHTML}
</script>
<!-- TODO(TailwindCSS): Change classes with utility components -->
<LeftSidebar>
<nav aria-labelledby="grid-left" class="flex justify-center space-x-4">
	<ul class="nav-groups 
    font-medium 
    px-3 py-2 
    text-slate-700 
    rounded-lg 
    hover:bg-slate-100 
    hover:text-slate-900">

      {#if matchParams === 'page' || false}
      sidebar.map(([child]) => (
        process.env.VITE_SUPABASE_PUBLIC_URL? + child.link
        ))
        {/if}
      </ul>
    </nav>
</LeftSidebar>