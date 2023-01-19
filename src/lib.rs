use photon_rs::PhotonImage;
use wasm_bindgen::{prelude::*};
use wee_alloc::WeeAlloc;

extern crate photon_rs;


#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello(text:&str){
    let output_text = format!("{} <----",text);
    alert(&output_text);
}

#[wasm_bindgen]
//source_img: &[u8]
pub fn new_resizer(source_img: &[u8],wdth:u32,hgth:u32)->Result<PhotonImage,JsError>{
    let  content = photon_rs::native::open_image_from_bytes(source_img).unwrap();
    let test = photon_rs::transform::resize(&content, wdth,hgth, photon_rs::transform::SamplingFilter::Gaussian);
    Ok(test)
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}