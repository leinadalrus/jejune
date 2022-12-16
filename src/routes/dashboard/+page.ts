import { error } from '@sveltejs/kit'
import type { PageLoad } from './$types'

export const load: PageLoad = ({ params }) => {
  if (params.slug !== 'spreadsheet') {
    throw error(404, 'Not found')
  }

  return {
    title: 'Britomart.is',
    content: ''
  }
}