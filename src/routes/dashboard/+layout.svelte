<script lang='ts'>
  import ImmutableModel from '../../models/faas/DashboardModel.svelte'
  import { beforeUpdate } from 'svelte'

  export let items: any = [
    { user: '', content: '', commentsCount: 1, timeAgo: Date.now() }
  ]
  export let inputs = [
    { id: 0, done: true, body: '' }
  ]
  $: url = items.type === 'ask' ? 
    `https://story.tandembytes.com/${items.id}` : 
      items.urls

  function commentBody () {
    const content = items.commentsCount
    return `${content} ${content === 1 ? 'comment' : 'comments'}`
  }

  function toggle (id: any) {
    let input = inputs.map(input => {
      if (input.id === id) {
        return {
          id,
          done: !input.done,
          body: input.body
        }
      }

      return input
    })
  }
</script>

{#each inputs as input}
  <ImmutableModel {input} on:click='{() => toggle(input.id)}'/>
{/each}

<article>
  <span>{items + 1}</span>
  <!-- svelte-ignore security-anchor-rel-noreferrer -->
  <h2><a href='{url}' target='_blank'>{items.title}</a></h2>
  <p class='meta'>
    <a href='#/item/{items.id}'>{commentBody()}</a> by 
    {items.user} 
    {items.timeAgo}
  </p>
</article>