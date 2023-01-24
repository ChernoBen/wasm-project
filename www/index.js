import init,{new_resizer} from "wasm";
import wasm from 'wasm/wasm_bg.wasm';

async function get_img_uint8(){

  const img = await fetch("https://cdn.pixabay.com/photo/2017/09/01/00/15/png-2702691_1280.png");
  const blolb = await img.blob();
  const array_buff = await blolb.arrayBuffer();
  return new Uint8Array(array_buff);
}


async function handleRequest(request){

    const data = await get_img_uint8();
    let width = new URL(request.url).searchParams.get("width");
    if (!width){width = 1280};
    let height = new URL(request.url).searchParams.get("height");
    if (!height) {height = 851};

    const wasm_m = await init(fetch(wasm).then(response=>response.arrayBuffer())).then(_=>{
      return new_resizer(data,width,height);
    });

    const img_wasm = wasm_m.get_base64();
    const html_content = `<img src='${img_wasm}'/>`;
    return new Response(html_content,{
        headers: {
          "content-type": "text/html;charset=UTF-8",
        },
      }
    );
}

addEventListener("fetch",event=>{
  event.respondWith(handleRequest(event.request));
});

