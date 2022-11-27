
function checkInlineInput () {
  var input = document.getElementsByTagName('input')
  if (!input.match(/^[\w+]$/))
    input.innerHTML = 'Your Input is invalid here! Change it!'

  console.error('Invalid input detected! Exiting ...')

  return
}

function forceHideErrorMsg () {
  document.getElementsByTagName('input').innerHTML = false
}

