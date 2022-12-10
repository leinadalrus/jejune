// import // ? ...
<script>
class PostData {
  constructor () {
    this.#userID
    this.#id
    this.#title
    this.#body
  }

  #userID = 0
  #id = 0
  #title = ''
  #body = ''
}

class PostDataProps {
  constructor () {
    this.#postData
  }

  #postData = new PostData()
}

function Post ({ title, body, id }) {
  return (
    <main>
      <article>
        <h2>{title}</h2>
        <p>{body}</p>
        <a href={`/post/${id}`}></a>
      </article>
    </main>
  )
}

export async function GetPost (id) {
  const response = await fetch(
    `https:/my.brio/posts/${id}`
  )
  const postData = await response.json()
  return postData
}

export async function GetPosts () {
  const response = await fetch(
    'https://my.brio/posts?_page=1'
  )
  const postList = await response.json()
  return postList
}
</script>