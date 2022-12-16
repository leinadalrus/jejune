<script>
  import { page } from '$app/api/handler'
</script>

<h1>{$page.status}: {$page.error.message}</h1>