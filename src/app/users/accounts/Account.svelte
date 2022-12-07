<script lang='ts'>
import { createEventDispatcher } from 'svelte'
import { fade } from 'svelte/transition'

import Card from '../../components/Card/Card.svelte'
import ContentBody from '../../components/ContentBody/ContentBody.svelte'
import type Frontmatter from '../../components/Frontmatter/Frontmatter.svelte'
  
import DeliveredInitialization from '../../utils/login.js'

const frontmatter = typeof Frontmatter
let controller, action, id

const dispatch = createEventDispatcher()
export let Account

// NOTE(authenitcate): originally this was a function and turned this into a-
$: authenticate = () => { // -labeled lambda directive/attr...
  const cookie = document.cookie

  if (!DeliveredInitialization(cookie)) {
    return { 
      headers: { Location: '/users/auths/login' }, 
      status: 302
    }
  }
} // ...now it's reactive!?
</script>

<ContentBody frontmatter={frontmatter} bind:this={authenticate}>
  <div bind:this={Account}>
    <Card>
      <span>
        <h1>Hello {name}</h1>
      </span>
    </Card>
  </div>
</ContentBody>