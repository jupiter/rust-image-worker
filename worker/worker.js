addEventListener("fetch", event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const { convert_image } = wasm_bindgen;
  await wasm_bindgen(wasm);

  try {
    const imageResponse = await fetch(
      "http://factorymethod.uk/FactoryMethod-Logo.png"
    );
    const data = await imageResponse.arrayBuffer();
    const output = convert_image(new Uint8Array(data));
    let res = new Response(output, { status: 200 });
    res.headers.set("Content-type", "image/png");
    return res;
  } catch (e) {
    let res = new Response(`Error (${e.message})`, { status: 200 });
    res.headers.set("Content-type", "text/html");
    return res;
  }
}
