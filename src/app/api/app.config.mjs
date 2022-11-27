export const SITE = {
  name: 'Brio',

  origin: 'https://britomart.com',
  basePathname: '/',

  // title: 'AstroWind — Your website with Astro + Tailwind CSS',
  // description: '🚀 AstroWind is a free and ready to start template to make your website using Astro and Tailwind CSS.',

  googleAnalyticsId: false // or "G-XXXXXXXXXX",
  // googleSiteVerificationId: 'orcPxI47GSa-cRvY11tUe6iGg2IO_RPvnA1q95iEM3M',
}

export const ARTICLE = {
  disabled: false,
  postsPerPage: 8,

  articles: {
    disabled: false,
    pathname: 'articles' // blog main path, you can change this to "articles" (/articles)
  },

  post: {
    disabled: false,
    pathname: '' // empty for /some-post, value for /pathname/some-post
  },

  category: {
    disabled: false,
    pathname: 'category' // set empty to change from /category/some-category to /some-category
  },

  tag: {
    disabled: false,
    pathname: 'tag' // set empty to change from /tag/some-tag to /some-tag
  }
}
