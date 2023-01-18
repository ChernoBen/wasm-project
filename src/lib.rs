use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello(text:&str){
    let output_text = format!("{} <----",text);
    alert(&output_text);
}


#[wasm_bindgen]
pub fn img_resize(source_img:Vec<u8>) -> Result<Vec<u8>,JsError> {
    let result = image::load_from_memory_with_format(&source_img,image::ImageFormat::Png);
    let dcd_img = result.unwrap();
    let resized = image::imageops::resize(
        &dcd_img,
        600,
        400,
        image::imageops::FilterType::Gaussian
    );
    //let _ = resized.save("./gato-output.png");
    //let test = resized.to_owned().into_vec();
    let response = resized.into_raw();
    alert("parece que deu bom");
    Ok(response)
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}