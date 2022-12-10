export const Get = (params, reqs) => {
  return {
    body: JSON.stringify({
      path: new URL(reqs.url).pathname
    })
  }
}

export const Put = (params, reqs) => {
  return {
    body: JSON.stringify({
      path: new URL(reqs.url).pathname
    })
  }
}

export const Post = (reqs) => {
  return {
    body: JSON.stringify({
      path: new URL(reqs.url).pathname
    })
  }
}

export const Del = (reqs) => { //  NOTE: Since delete is a reserved word in JavaScript, export a del function to match the delete method.
  return {
    body: JSON.stringify({
      path: new URL(reqs.url).pathname
    })
  }
}