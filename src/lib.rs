use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use image::{ImageBuffer, Rgba};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello(text:&str){
    let output_text = format!("{} <----",text);
    alert(&output_text);
}

//ImageBuffer<Rgba<u8>, Vec<u8>>
#[wasm_bindgen]
pub fn img_resize(img_vec: Vec<u8>) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>,JsValue> {
        
    let resized = image::imageops::resize(
        &img_vec,
        151,
        201,
        image::imageops::FilterType::Lanczos3
    );
    //resized.save("mario-ouput.png")?;

    Ok(resized)
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}