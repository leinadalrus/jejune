<script>
import Card from '../../components/Card/Card.astro'
import EditorLayout from '../../../layouts/EditorLayout.astro'
import type { Props } from '../../components/Frontmatter/Frontmatter.astro'
import FooterMeta from '../../components/Footer/FooterMeta.svelte'

let { frontmatter } = typeof Frontmatter
<script>

<EditorLayout frontmatter={frontmatter}>
    <ul role='list' class='link-card-grid'>
      <Card
        href='https://my.brio/featured'
        title='Featured'
        body='Discover new and popular reads.'
      />
      <slot />
    </ul>
</EditorLayout>

<FooterMeta>
  <a href='https://my.br.io' target='_blank' rel='noopener noreferrer'>
    Powered by
    <span>
      <img src='public/favicon.svg' alt='Astro Logo' width={72} height={16} />
    </span>
  </a>
</FooterMeta>