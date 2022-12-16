<script lang='ts'>
  import type { PageData } from './$types'
  import ContentBody from '../components/ContentBody/ContentBody.svelte'
  import EditorLayout from '../../layouts/EditorLayout.svelte'
  export let data: PageData
</script>

<ContentBody>
  <EditorLayout>
    <p>{data.title}</p>
    <div>{@html data.content}</div>
  </EditorLayout>
</ContentBody>