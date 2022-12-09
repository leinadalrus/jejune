<script lang='ts'>
  import type { PageData } from './$types.d.ts'
  import ContentBody from '../../../app/components/ContentBody/ContentBody.svelte'
  import EditorLayout from '../../../layouts/EditorLayout.svelte'
  export let data: PageData
</script>

<ContentBody>
  <EditorLayout>
    <p>{data.title}</p>
    <div>{@html data.content}</div>
  </EditorLayout>
</ContentBody>