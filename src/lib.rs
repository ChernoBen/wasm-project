use photon_rs::PhotonImage;
use wasm_bindgen::{prelude::*};
use web_sys::ImageData;
use wee_alloc::WeeAlloc;
use image::io::Reader;
use std::{io::{Cursor}};

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
    let content = photon_rs::native::open_image_from_bytes(source_img).unwrap();
    let test = photon_rs::PhotonImage::new(content.get_raw_pixels(), wdth, hgth);
    Ok(test)
}

#[wasm_bindgen]
pub fn img_resize(source_img:Vec<u8>) -> Result<Vec<u8>,JsError> {
    let cursor = Cursor::new(source_img);
    let reader = Reader::new(cursor).with_guessed_format()
    .expect("This will never fail using Cursor");
    let img_reader = reader.decode().expect("Failed to read image");
    let resized = image::imageops::resize(
        &img_reader,
        600,
        400,
        image::imageops::FilterType::Gaussian
    );
    let response = resized.into_raw();
    
    //let unwrapped_vec = img_reader.into_rgb8().into_raw();
    alert("parece que deu bom");
    Ok(response)
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}