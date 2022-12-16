<script lang='ts'>
/** @type {import('./$types').RequestHandler} */
import { json } from '@sveltejs/kit'

// import { createEventDispatcher } from 'svelte'

import Card from '../../components/Card/Card.svelte'
import ContentBody from '../../components/ContentBody/ContentBody.svelte'
  
import DeliveredInitialization from '../auths/Login.svelte'

// const dispatch = createEventDispatcher() // just make this into a reactive function [!]
export let Account

// NOTE(authenitcate): originally this was a function and turned this into a-
$: authenticate = (event: any)=>{ // -labeled lambda directive/attr...
  const cookie = document.cookie
  if (!new DeliveredInitialization(cookie)) {
    return json({ 
      headers: { Location: '/users/auths/login' }, 
      status: 302
    })
  }
   
    return json({
      // retrieve a specific header
      userAgent: event.request.headers.get('user-agent')
    })
} // ...now it's reactive!?
</script>

<ContentBody>
  <div bind:this={Account}>
    <Card>
      <span>
        <h1>Hello {name}</h1>
      </span>
    </Card>
  </div>
</ContentBody>