<script>
import { createEventDispatcher } from 'svelte'
import { fade } from 'svelte/transition'

import Card from '../../components/Card/Card.astro'
import ContentBody from '../../components/ContentBody/ContentBody.astro'
  
import DeliveredInitialization from '../../utils/login.js'

const { frontmatter } = Astro.props
const { controller, action, id } = Astro.params

const dispatch = createEventDispatcher()
export let Account

// NOTE(authenitcate): originally this was a function and turned this into a-
$: authenticate = () => { // -labeled lambda directive/attr...
  const cookie = Astro.request.headers.get('cookie')

  if (!DeliveredInitialization(cookie)) {
    return Astro.redirect('/users/auths/login')
  }
} // ...now it's reactive!?
</script>

<ContentBody frontmatter={frontmatter} bind:this={authenticate}>
  <div bind:this={Account}>
    <Card>
    
    </Card>
  </div>
</ContentBody>