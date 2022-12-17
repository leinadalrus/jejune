<script lang='ts'>
  import { beforeUpdate } from 'svelte'
  import type { LayoutData } from './$types'
  import DashboardModel from '../../models/faas/DashboardModel.svelte'
  import './app.css'

  export let items: any = [
    { user: '', content: '', commentsCount: 1, timeAgo: Date.now() }
  ]
  export let inputs = [
    { id: 0, done: true, body: '' }
  ]
  $: url = items.type === 'ask' ? 
    `https://story.tandembytes.com/${items.id}` : 
      items.urls

  let data: LayoutData

  function commentBody (): string {
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
  <input type="checkbox" bind:group={input} on:click='{() => toggle(input.id)}'/>
  {input}
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