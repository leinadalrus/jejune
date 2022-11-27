<script lang='ts'>
import type Frontmatter from '../Frontmatter/Frontmatter.svelte'

let frontmatter: Frontmatter
</script>
<head>
	<meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <meta name="Brio: Markdown web novel - story writing"
  content="Write stories with Brio, 
  and with as much fidelity with Markdown format!"/>
	<meta name="generator" content={Astro.generator} />
	<link rel="icon" type="image/svg+xml" href="/favicon.svg" />
	<title>{frontmatter.title}</title>
</head>

<!-- Preload Fonts -->
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link
  href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital@0;1&display=swap"
  rel="stylesheet"
/>

<nav class="flex justify-center space-x-4">
  <a class="font-medium 
  px-3 
  py-2 
  text-slate-700 
  rounded-lg 
  hover:bg-slate-100 
  hover:text-slate-900" href="/index/">Home</a>
  <a class="font-medium 
  px-3 
  py-2 
  text-slate-700 
  rounded-lg 
  hover:bg-slate-100 
  hover:text-slate-900" href="/users/documents/BaseDocumentModel">My Stories</a>
  <a class="font-medium 
  px-3 
  py-2 
  text-slate-700 
  rounded-lg 
  hover:bg-slate-100 
  hover:text-slate-900" href="/users/User/">Me</a>
  <h1>{frontmatter.title} by {frontmatter.author}</h1>
  <!-- <slot /> -->
  <p>Written on: {frontmatter.date}</p>
</nav>

<!-- This is intentionally inlined to avoid FOUC -->
<script is:inline>
  const root = document.documentElement
  const theme = localStorage.getItem('theme')
  if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    root.classList.add('theme-dark')
  } else {
    root.classList.remove('theme-dark')
  }
</script>
