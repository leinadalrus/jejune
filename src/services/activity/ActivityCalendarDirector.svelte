<script lang='js'>
  import fs from 'fs'

  class ActivityDirectorService {
    #trackedFiles = new FileHandle()
    #fileContents = new Set()

    update () {
      input => {
        this.#trackedFiles = fetch(process.env.VITE_SUPABASE_PUB_URL+`/{user}?docs=${id}`)
        this.#fileContents.add(input)
        fs.writeFile(this.#trackedFiles, this.#fileContents, 'utf8')
      }
    }

    destroy () {
      return async input => {
        this.#trackedFiles = fetch(process.env.VITE_SUPABASE_PUB_URL+`/{user}?docs=${id}`)
        this.#fileContents.delete(input)
        fs.writeFile(this.#trackedFiles, this.#fileContents, 'utf8')
      }
    }
  }

</script>