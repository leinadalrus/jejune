import { get } from 'svelte/store'
import { tweened, spring } from 'svelte/motion'

const SubscribeClient = get(() => {
  return store // ?...
})