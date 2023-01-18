import init,{hello,img_resize} from "../pkg";

init().then(_=>{
    hello("testandooo!!!"); 
    console.log("show show");
    try{
        var xhr = new XMLHttpRequest();
        xhr.open("GET", "https://cdn.pixabay.com/photo/2017/09/01/00/15/png-2702691_1280.png", true);
        xhr.responseType = "arraybuffer";
        
        xhr.onload = function (e) {
          try{
            var arrayBufferView = new Uint8Array(this.response);
            var content = img_resize(arrayBufferView);
            console.log(arrayBufferView);
            var blob = new Blob([content],{ type: "image/png"});
            //var urlCreator = URL;
            var imageUrl = URL.createObjectURL(blob);
            var img = document.querySelector("#photo");
            img.src = imageUrl;

          }catch(error){
            console.log(error.constructor.name);
          }
        };

        xhr.send();
        
    }catch(error){
        if (error){
            console.error(error);
        }else{
            console.log("kd o erro???")
        }
        
    }
});