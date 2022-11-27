export default function fade (element) {
  requestAnimationFrame(() => {
    element.style.transition = 'none'
    element.style.color = 'hex(#FF526A)'
    element.style.backgroundColor = 'hex(#4c4244)'

    setTimeout(() => {
      element.style.transition = 'color 1s, background 1s'
      element.style.color = ''
      element.style.backgroundColor = ''
    })
  })
}

export function next () {
  $: next = items.type === 'next' ? 
  `https://story.brio.app/\${`+ items.id++ + `}` :
    items.next
}

export function previous () {
  $: next = items.type === 'next' ? 
  `https://story.brio.app/\${`+ items.id-- + `}` : 
    items.next
}

export function unknown () {
  $: unknown = items.type === 'undefined' ? 
  `https://story.brio.app/404` : 
    items.undefined
}