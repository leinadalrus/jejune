<script lang='ts'>
import type Frontmatter from '../Frontmatter/Frontmatter.svelte'

export let frontmatter: Frontmatter
</script>

<article id="article" class="content">
	<section class="main-section">
		<h1 class="content-title" id="overview">{frontmatter.title}</h1>
    <img src="${frontmatter.image.src}" 
      alt="${frontmatter.image.alt}" 
      srcset="">
		<p>{frontmatter.description}</p>
	</section>
</article>