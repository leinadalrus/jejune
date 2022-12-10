import { error } from '@sveltejs/kit'
import { json } from '@sveltejs/kit'
import type { RequestHandler } from './$types'

export const GET: RequestHandler = (url: any) => {
  if (url === undefined)
    throw error(400, '\'URL\': is undefined ...')

  return new Response(url)
}

export const POST: RequestHandler = async (request: any) => {
  const { reqs } = await request.json()
  return json(reqs)
}