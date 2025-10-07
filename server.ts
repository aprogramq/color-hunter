Bun.serve({
  port:3001,
  fetch(req){
    console.log(req.url);
    return new Response("Hello, world!");
  }
})

