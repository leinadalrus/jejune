<script>
import EditorLayout from '../../../layouts/EditorLayout.astro'
import ContentBody from '../../components/ContentBody/ContentBody.astro'
import type { Props } from '../../components/Frontmatter/Frontmatter.astro'

const { frontmatter } = typeof Frontmatter
</script>

<EditorLayout frontmatter={frontmatter}>
  <ContentBody frontmatter={frontmatter}>
    <slot />
  </ContentBody>
</EditorLayout>
