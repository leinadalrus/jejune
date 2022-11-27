import { APIRoute } from 'astro'

const Usernames = ([username]) => {
  return [username]
}

export const APIRouteUsernames = (params, reqs) => {
  const id = params.id

  if (!id) {
    return new Response(null, {
      status: 404,
      statusText: 'Not Found'
    })
  }

  return {
    body: JSON.stringify({
      name: Usernames[id],
      path: new URL(reqs.url).pathname
    })
  }
}
