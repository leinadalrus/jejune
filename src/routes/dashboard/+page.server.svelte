<script lang='ts'>
  import * as db from '$lib/server/database'
  import type { PageData } from './$types'
  import type { PageServerLoad } from './$types'
  import ContentBody from '../components/ContentBody/ContentBody.svelte'
  import EditorLayout from '../../layouts/EditorLayout.svelte'
  export let data: PageData

  export const load = (async ({ params: any }) => {
    return {
      post: await db.getPost(params.slug)
    }
  }) satisfies PageServerLoad
</script>

<ContentBody>
  <EditorLayout>
    <p>{data.title}</p>
    <div>{@html data.content}</div>
  </EditorLayout>
</ContentBody>