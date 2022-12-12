<script lang='ts'>
import { VITE_SUPABASE_PUB_URL } from '$env/static/private';

import ContentBody from '../routes/components/ContentBody/ContentBody.svelte'
import Footer from '../routes/components/Footer/FooterMeta.svelte'
import Frontmatter from '../routes/components/Frontmatter/Frontmatter.svelte'

type Props = {
  frontmatter: Frontmatter
}

let frontmatter = typeof Frontmatter
const currentPage = VITE_SUPABASE_PUB_URL
const currentFile = `src/routes${currentPage.replace(/\/$/, '')}.md`

const pages = {
  undefined: {
    title: 'Brio',
    text: 'Hello! And Welcome to Brio!',
  },
  'homepage': {
    title: 'My Homepage',
    text: 'This is your homepage',
  },
  'dashboard': {
    title: 'Dashboard for all highlighted and featured',
  },
  'editor': {
    title: 'My Editor',
    text: 'Your stories to write',
  },
  'Analytics': {
    title: 'My Analytics',
    text: 'View your statistics',
  },
  'Settings': {
    title: 'My Settings',
    text: 'Change Settings',
  },
  'profile': {
    title: 'My Profile',
    text: 'Manage Profile',
  }
}

// const page = pages[slug]
if (!pages) new Request(VITE_SUPABASE_PUB_URL)
</script>

<html dir={frontmatter.dir ?? 'ltr'} lang={frontmatter.lang ?? 'en-uk'} class="initial">
  <HeaderMeta frontmatter={frontmatter} />
  <title>
    {frontmatter.title}
  </title>

  <body class="md:container md:mx-auto px-4 overscroll-auto hover:overscroll-contain">
    <article class="grid grid-cols-1">
      <main>
        <ContentBody frontmatter={frontmatter} />
      </main>

      <aside>
        <div class="py-8 px-8 max-w-sm mx-auto bg-white dark:bg-slate-800 rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6">
          <img class="block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0" src="/img/daniel-surla.jpg" alt="Woman's Face">
          <div class="text-center space-y-2 sm:text-left">
            <div class="space-y-0.5">
              <p class="text-lg text-slate-900 dark:text-white font-semibold">
                Daniel Surla
              </p>
            <p class="text-slate-500 dark:text-slate-400 mt-2 text-sm">
              Intellectual Simpleton
            </p>
          </div>
          <button class="px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border border-purple-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2">Message</button>
        </div>
      </div>
    </aside>
    </article>
  </body>

  <footer>
    <Footer path={currentFile} />
  </footer>
</html>