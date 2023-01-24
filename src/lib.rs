use photon_rs::PhotonImage;
use wasm_bindgen::{prelude::*};
use wee_alloc::WeeAlloc;
use std::panic;

extern crate photon_rs;
extern crate console_error_panic_hook;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


#[wasm_bindgen]
//source_img: &[u8]
pub fn new_resizer(source_img: &[u8],wdth:u32,hgth:u32)->Result<PhotonImage,JsError>{
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let  content = photon_rs::native::open_image_from_bytes(source_img).unwrap();
    let resized_img = photon_rs::transform::resize(&content, wdth,hgth, photon_rs::transform::SamplingFilter::Gaussian);
    Ok(resized_img)
}

