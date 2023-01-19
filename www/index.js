import init,{hello,new_resizer} from "../pkg";


// init().then(async _=>{

// });

hello("testandooo!!!");
const img = await fetch("https://cdn.pixabay.com/photo/2017/09/01/00/15/png-2702691_1280.png");
const blolb = await img.blob();
const array_buff = await blolb.arrayBuffer();
const data = new Uint8Array(array_buff);
const width = 1080;
const height = 720;
const res = new_resizer(data,width,height);
console.log(res.get_base64());
const t2 = Uint8Array.from(window.atob(res.get_base64().replace(/^data[^,]+,/,'')), v => v.charCodeAt(0));
var blb = new Blob([t2],{ type: "image/png"});
var imageUrl = URL.createObjectURL(blb);
var page = document.querySelector("#photo");
page.src = imageUrl;