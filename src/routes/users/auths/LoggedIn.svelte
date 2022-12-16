<script lang='js' context='module'>
  export class LoggedIn {
    authorize () {
      return (svelteHTML) => {
        const markup = within(svelteHTML)
        const loginButton = markup.getByRole('button', { name: /Log in/i })
        userEvent.click(loginButton)
      }
    }
    
    rememberMe = false
  }
</script>

{@html Component}
<Object bind:this={LoggedIn} />