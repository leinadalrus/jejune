<script lang='ts'>

import { getLanguageFromURL, SIDEBAR } from '../Sidebar/Sidebar.svelte'
type Props = {
  currentPage: string
}

const { currentPage } = Astro.props as Props
const currentPageMatch = currentPage.endsWith('/') 
  ? currentPage.slice(1, -1) 
  : currentPage.slice(1)

const langCode = getLanguageFromURL(currentPage);
const sidebar = SIDEBAR[langCode]
</script>
<!-- TODO(TailwindCSS): Change classes with utility components -->
<nav aria-labelledby="grid-left" class="flex justify-center space-x-4">
	<ul class="nav-groups 
    font-medium 
    px-3 py-2 
    text-slate-700 
    rounded-lg 
    hover:bg-slate-100 
    hover:text-slate-900">
		{Object.entries(sidebar).map(([header, children]) => (
			<li>
				<div class="nav-group">
					<h2>{header}</h2>
					<ul>
						{children.map((child) => {
							const url = Astro.site?.pathname + child.link;
							return (
								<li class="nav-link">
									<a href={url} aria-current={currentPageMatch === child.link ? 
                    'page' : false}>
										{child.text}
									</a>
								</li>
							);
						})}
					</ul>
				</div>
			</li>
		))}
	</ul>
</nav>