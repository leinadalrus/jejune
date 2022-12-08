<script lang='ts'>
import Card from '../../components/Card/Card.svelte'
import EditorLayout from '../../../layouts/EditorLayout.svelte'
import Frontmatter from '../../components/Frontmatter/Frontmatter.svelte'
import FooterMeta from '../../components/Footer/FooterMeta.svelte'

let frontmatter = typeof Frontmatter
/**
     * @type {any}
     */
let path: string
</script>

<EditorLayout frontmatter={frontmatter}>
    <ul class='link-card-grid'>
      <Card />
      <slot />
    </ul>
</EditorLayout>

<FooterMeta path={path}>
  <a href='https://my.britomart.is' target='_blank' rel='noopener noreferrer'>
    Powered by
    <span>
      <img src='public/favicon.svg' alt='Astro Logo' width={72} height={16} />
    </span>
  </a>
</FooterMeta>