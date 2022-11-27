---
export type Frontmatter = {
  title: string
  author: string
  date: Date
  draft: string
  description: string
  layout: string
  image?: { 
    src: string
    alt: string 
  }
	dir?: 'ltr' | 'rtl'
	ogLocale?: string
	lang?: string
}

export interface Props {
	frontmatter: Frontmatter
}

export const { frontmatter } = Astro.props
export const { controller, action, id } = Astro.params
---