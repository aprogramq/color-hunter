Bun.serve({
  port:3001,
  fetch(req){
    return new Response("Hello, world!");
  }
})

