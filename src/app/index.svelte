---
import Card from './components/Card/Card.astro'
import EditorLayout from '../layouts/EditorLayout.astro'
import type { Props } from './components/Frontmatter/Frontmatter.astro'

// const cookie = Astro.request.headers.get('cookie')
Astro.request.headers.get('cookie')

const { frontmatter } = Astro.props as Props
---

<EditorLayout frontmatter={frontmatter}>
  <main>
    <h1>Welcome to <span class='text-gradient'>Astro</span></h1>
    <p class='instructions'>
      'Write stories with Brio, and with as much fidelity with Markdown format!'
    </p>
    <ul role='list' class='link-card-grid'>
      <Card
        href='https://my.brio'
        title='Home'
        body='Hello, World! And welcome to Br.io!</a>'
      />
      <Card
        href='https://my.brio/write'
        title='Write'
        body='With Markdown, you can write stories with a great deal of control.'
      />
      <Card
        href='https://my.brio/learn'
        title='Learn'
        body='Learn how to write good stories'
      />
      <Card
        href='https://my.brio/featured'
        title='Featured'
        body='Discover new and popular reads.'
      />
    </ul>
  </main>
</EditorLayout>

<footer>
  <a href='https://my.br.io' target='_blank' rel='noopener noreferrer'>
    Powered by
    <span>
      <img src='public/favicon.svg' alt='Astro Logo' width={72} height={16} />
    </span>
  </a>
</footer>