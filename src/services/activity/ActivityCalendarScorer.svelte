<script lang='js'>
  import fs, { appendFile } from 'fs'

  const ScoresTracker = {
    trackedFile: fs.watchFile(undefined),
    trackedFile: fs.fs(),
  }

  export const ActivityCalendarScorer = (trackedFile, trackedChanges) => {
    let fileHandle = fs.appendFile(trackedFile, trackedChanges)
    return { 
      trackedFile: fs.watchFile(fileHandle),
      trackedChanges: fs.fstat() 
    }
  }
</script>