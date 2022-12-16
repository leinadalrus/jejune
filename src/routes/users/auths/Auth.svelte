<script>
  import LoggedIn from 'LoggedIn.svelte'
  import Enumerables from '../../../models/faas/Enumerables.svelte'
  import HeaderMeta from '../../components/Header/HeaderMeta.astro';
  import FooterMeta from '../../components/Footer/FooterMeta.astro'

  let isEditing = false
</script>


<HeaderMeta />
  
  <form action='' method='post'>
    <input type='email' name='userEmailaddress' id='user-emailaddress'>
    <input type='password' name='userPassword' id='user-password'>
    
    <button type='submit' name='loginButton' id='login-button'>Login</button>

    <input type='checkbox' name='' id='' bind:checked={LoggedIn.rememberMe}>
  </form>
  
<FooterMeta />

<style></style>