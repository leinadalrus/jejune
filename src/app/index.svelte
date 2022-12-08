<script lang='ts'>
import Card from '../components/Card/Card.svelte'
import EditorLayout from '../../layouts/EditorLayout.svelte'
import type Frontmatter from './../components/Frontmatter/Frontmatter.svelte'

const frontmatter = Frontmatter
</script>

<EditorLayout frontmatter={frontmatter}>
  <main>
    <h1>Welcome to <span class='text-gradient'>Astro</span></h1>
    <p class='instructions'>
      'Write stories with Britomart Information Systems, and with as much fidelity with Markdown format!'
    </p>
    <ul class='link-card-grid'>
      <Card
        href='https://britomart.is'
        title='Home'
        body='Hello, World! And welcome to Br.io!</a>'
      />
      <Card
        href='https://britomart.is/write'
        title='Write'
        body='With Markdown, you can write stories with a great deal of control.'
      />
      <Card
        href='https://britomart.is/learn'
        title='Learn'
        body='Learn how to write good stories'
      />
      <Card
        href='https://britomart.is/featured'
        title='Featured'
        body='Discover new and popular reads.'
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
