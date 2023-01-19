import init,{hello,img_resize,new_resizer} from "../pkg";


init().then(async _=>{
    hello("testandooo!!!"); 
    console.log("show show");

    const img = await fetch("https://cdn.pixabay.com/photo/2017/09/01/00/15/png-2702691_1280.png");
	const blolb = await img.blob();
	const array_buff = await blolb.arrayBuffer();
	const data = new Uint8Array(array_buff);
    //console.log(typeof data);
    //const res = new_resizer(data);
    const width = 1281;
    const proportion = 1.5041128085;
    const default_h = 851;
    const aspectRatio = width/default_h
    const height = default_h/aspectRatio;

    const res = new_resizer(data,width,height);
    console.log(res.get_base64());
    const t2 = Uint8Array.from(window.atob(res.get_base64().replace(/^data[^,]+,/,'')), v => v.charCodeAt(0));
    var blb = new Blob([t2],{ type: "image/png"});
    var imageUrl = URL.createObjectURL(blb);
    var page = document.querySelector("#photo");
    page.src = imageUrl;


    // try{
    //     var xhr = new XMLHttpRequest();
    //     xhr.open("GET", "https://cdn.pixabay.com/photo/2017/09/01/00/15/png-2702691_1280.png", true);
    //     xhr.responseType = "arraybuffer";
    //     xhr.onload = function (e) {
    //       try{
    //         var arrayBufferView = new Uint8Array(this.response);
    //         var content = img_resize(arrayBufferView);
    //         content = new Uint8Array(content);
    //         var content2 = new_resizer(arrayBufferView.slice());
    //         var img_data = content2.get_image_data().data;
    //         console.log(content2.constructor.name);
    //         console.log(content2.get_image_data().data);
    //         var blob = new Blob([img_data],{ type: "image/png"});
    //         var imageUrl = URL.createObjectURL(blob);
    //         var img = document.querySelector("#photo");
    //         img.src = imageUrl;
    //       }catch(error){
    //         console.error(error);
    //       }
    //     };
    //     xhr.send();

    // }catch(error){
    //     if (error){
    //         console.error(error);
    //     }else{
    //         console.log("kd o erro???")
    //     }
        
    // }
});