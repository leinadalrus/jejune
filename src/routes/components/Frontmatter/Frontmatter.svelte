<script lang='ts' context='module'>
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
  url: string
	dir?: 'ltr' | 'rtl'
	ogLocale?: string
	lang?: string
}
</script>