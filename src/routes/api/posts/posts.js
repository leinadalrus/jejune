function getContentBodyPost (postID) {
  const { frontmatter, Content, file } = postID
  const ID = file
    .split('/')
    .pop()
    .split('.')
    .shift()

  return {
    id: ID,
    date: frontmatter.date,
    draft: frontmatter.draft,
    title: frontmatter.title,
    description: frontmatter.description,
    author: frontmatter.author,
    Content: Content
  }
}

async function loadContentBodyPost () {
  const POSTS = import.meta.glob(
    ['~/../data/blog/**/*.md', '~/../data/blog/**/*.mdx'],
    {
      eager: true
    }
  )

  const NORMALISED = Object.keys(posts).map(async key => {
    const P = await posts[key]
    return await getContentBodyPost(P)
  })

  const RESULTS = (await Promise.all(NORMALISED))
    .sort((a, b) => new Date(b.date).valueOf() - new Date(a.date).valueOf())
    .filter(post => !post.draft)

  return RESULTS
}

async function fetchContentBodyPost () {
  _posts = _posts || loadContentBodyPost()
  return await _posts
}

async function findContentBodyPostID (ids) {
  if (!Array.isArray(ids)) return []

  const POSTS = await fetchContentBodyPost()

  return ids.reduce(function (r, id) {
    POSTS.some(function (post) {
      return id === post.id && r.push(post)
    })

    return r
  }, [])
}
