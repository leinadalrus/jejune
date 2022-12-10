import { createClient } from '@supabase/supabase-js'
import { supabaseURL, supabaseKey, supabase } from 'supabaseClient.js'

async function handleRequest (request) {
  const cl = createClient(supabaseURL, supabaseKey, {
    fetch: fetch.bind(globalThis)
  })

  const { data, error } = await cl.from('Documents').select('documentID')

  if (error) {
    console.log(error)
    return new Response(error.message || error.toString(), {
      status: 500
    })
  }

  return new Response(JSON.stringify(data), {
    status: 200,
    headers: { 'Content-Type': 'application/json' }
  })
}

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})
