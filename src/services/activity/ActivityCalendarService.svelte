<script lang='js'>
  import 'ActivityCalendarDirector.svelte'
  import 'ActivityCalendarScorer.svelte'
  import Enumerables from '../../models/faas/Enumerables.svelte'
  import { Calendar } from '@d3/calendar'

  $: trackedFiles = JSON.stringify(
    FileAttachment(process.env.VITE_SUPABASE_PUB_URL 
      + `/${user}/public/resources/.track/${filename} + ${id}`))

  export let ContributionsChart = Calendar(trackedFiles, {
    x: d => d.Date,
    y: (d, i, data) => i > 0 ? (d.Close - data[i - 1].Close) / data[i - 1].Close : NaN,
    yFormat: '+%',
    weekday,
    width
  })
</script>

<Enumerables>
  <svelte:component this={ContributionsChart}/>
</Enumerables>

<style>

</style>