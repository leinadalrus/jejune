import type { LayoutLoad } from './$types'

export const prerender = true;
export const csr = true;
 
export const load: LayoutLoad = () => {
  return {
    sections: [
      { slug: 'profile', title: 'Profile' },
      { slug: 'notifications', title: 'Notifications' }
    ]
  };
}