<script lang='ts'>
import Card from './components/Card/Card.svelte'
import EditorLayout from '../layouts/EditorLayout.svelte'
import Frontmatter from './components/Frontmatter/Frontmatter.svelte'
import { VITE_SUPABASE_PUB_URL } from '$env/static/private';
</script>

<EditorLayout>
  <main>
    <h1>Welcome to <span class='text-gradient'>Astro</span></h1>
    <p class='instructions'>
      'Write stories with Britomart Information Systems, and with as much fidelity with Markdown format!'
    </p>
    <ul class='link-card-grid'>
      <Card
        url={VITE_SUPABASE_PUB_URL+'/index'}
        title={'Britomart!'}
        body={'Hello, World! And welcome to Br.io!</a>'}
      />
      <Card
        url='https://britomart.is/write'
        title='Write'
        body='With Markdown, you can write stories with a great deal of control.'
      />
      <Card
        url={VITE_SUPABASE_PUB_URL+'/learn'}
        title={'Learn'}
        body={'Learn how to write good stories'}
      />
      <Card
        url={VITE_SUPABASE_PUB_URL+'/featured'}
        title={'Featured'}
        body={'Discover new and popular reads.'}
      />
    </ul>
  </main>
</EditorLayout>

<footer>
  <a href='https://britomart.is' target='_blank' rel='noopener noreferrer'>
    Powered by
    <span>
      <img src='static/favicon.svg' alt='Svelte Logo' width={72} height={16} />
    </span>
  </a>
</footer>
