<script lang="ts">
export function DeliveredInitialization () {
  let admin = sessionStorage.admin
  admin = JSON.stringify({
    id: 0,
    role: '__SESSION_ADMIN_EX',
    status: 'LOGIN_SYSTEM_FLAG_OK',
    username: admin,
  })

  let user = sessionStorage.user
  user = JSON.stringify({
    id: 1,
    role: '__SESSION_USER_SH',
    status: 'LOGIN_SYSTEM_FLAG_OK',
  })

  sessionStorage.setItem(sessionStorage.admin.id, admin)
  sessionStorage.setItem(sessionStorage.user.id, user)

  return 0
}

$: form = (parameter: any) => {
  let form = document.getElementsByTagName('form')
  let userInput = JSON.stringify(form.namedItem('input'))
  let event = window.addEventListener
  while (event) {
    for (let i = 0; i < 2; i++) {
      parameter = userInput[i]
    }
  }

  event = window.removeEventListener
}

function observed () {
  let key = sessionStorage.user.id

  form(key)

  return key
}

function itemized () {
  let val = {
    id: sessionStorage.user.id,
    role: sessionStorage.user.role,
    username: sessionStorage.user.username,
    password: sessionStorage.user.password
  }

  form(val)

  return JSON.stringify(val)
}

function ParseInitialized () {
  let user = sessionStorage.user
  return JSON.parse(user)
}

function refresh () {
  check()
  sessionStorage.setItem(observed(), itemized())
}

function destroy () {
  sessionStorage.user = delete sessionStorage.user
}

function check () {
  DeliveredInitialization()
  ParseInitialized()

  for (let i = 0; i < sessionStorage.length; i++) {
    switch (sessionStorage.user.role) {
      case 'LOGIN_SYSTEM_FLAG_OK':
        if (sessionStorage.key(i) === sessionStorage.user.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_OK'
        }

      case 'LOGIN_SYSTEM_FLAG_WARN':
        if (sessionStorage.key(i) === sessionStorage.user.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_WARN'
        }

      case 'LOGIN_SYSTEM_FLAG_ERROR':
        if (sessionStorage.key(i) === sessionStorage.user.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_ERROR'
          destroy()
        } else { refresh() }

      default:
        if (sessionStorage.key(i) === sessionStorage.user.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_ERROR'
          console.log('let localStorage.user -> \'LOGIN_SYSTEM_FLAG_ERROR\'')
          destroy()
        }
    }
  }

  for (let j = 0; j < sessionStorage.length; j++) {
    switch (sessionStorage.admin.role) {
      case 'LOGIN_SYSTEM_FLAG_OK':
        if (sessionStorage.key(j) === sessionStorage.admin.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_OK'
        }

      case 'LOGIN_SYSTEM_FLAG_WARN':
        if (sessionStorage.key(j) === sessionStorage.admin.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_WARN'
        }

      case 'LOGIN_SYSTEM_FLAG_ERROR':
        if (sessionStorage.key(j) === sessionStorage.admin.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_ERROR'
          destroy()
        } else { refresh() }

      default:
        if (sessionStorage.key(j) === sessionStorage.admin.id) {
          sessionStorage.user.status = 'LOGIN_SYSTEM_FLAG_ERROR'
          console.log('let localStorage.admin -> \'LOGIN_SYSTEM_FLAG_ERROR\'')
          destroy()
        } else { refresh() }
    }
  }
}
</script>