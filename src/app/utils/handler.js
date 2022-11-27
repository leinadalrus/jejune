class Handler {
  constructor () {}

  lookup (filepath) {
    let webhooks = JSON.stringify("./data/json/webhooks.json") // json path regx
    array.forEach(element => {
      element = filepath
      window.location.replace(webhooks.directories + JSON.stringify(element))
    })
  }

  remember () {
    let checkbox = document.getElementById('remember').checked
    if (checkbox) {
      localStorage.setItem('username', this.#usernameElement.values())
      localStorage.setItem('password', this.#passwordElement.values())
      checkbox = true
      localStorage.setItem('isRemembered', checkbox)
    } else localStorage.clear()
  }

  loadcached () {
    let checkbox = document.getElementById('remember').checked
    if (localStorage.getItem('isRemembered')) {
      this.#usernameElement.values = localStorage.getItem('username')
      this.#passwordElement.values = localStorage.getItem('password')
      checkbox = true
    }
  }

  #usernameElement = document.getElementsByName('username')
  #passwordElement = document.getElementsByName('password')
  #redirectionButton = window.getElementsByClassName('btn-link')
}

/** TODO(Wildcard): 
 *
 * @classdesc
 * @funcdesc
 * @params filepath
 * @var jsonval
 * using the wildcard operation with RegEx, you can do: "\/*\/**"---
 * -> to make it check for files within the root directory, and then into---
 * -> -> its subdirectories.
 * We can achieve this by using JSON file handling and I/O (input-output) as---
 * -> well.
 * 
 * 
 * 
*/
