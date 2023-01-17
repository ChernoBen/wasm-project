import init,{hello,img_resize} from "../pkg";

init().then(_=>{
    hello("testandooo!!!"); 
    console.log("show show");
    try{
        const content = img_resize("./mario.png");
        console.log(typeof content);
    }catch(error){
        console.error(error);
    }
});