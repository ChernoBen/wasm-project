import init,{hello,img_resize} from "../pkg";

init().then(_=>{
    hello("testandooo!!!"); 
    console.log("show show");
    try{
        const content = img_resize("./mario.png");
        console.log(typeof content);
    }catch(error){
        if (error){
            console.error(error);
        }else{
            console.log("kd o erro???")
        }
        
    }
});