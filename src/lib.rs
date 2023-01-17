use try_catch::catch;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use image::{io::Reader as ImageReader};
use std::{*, error::Error};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello(text:&str){
    let output_text = format!("{} <----",text);
    alert(&output_text);
}

// //ImageBuffer<Rgba<u8>, Vec<u8>>
// #[wasm_bindgen]
// pub fn img_resize(img_vec: Vec<u8>) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>,JsValue> {
        
//     let resized = image::imageops::resize(
//         &img_vec,
//         151,
//         201,
//         image::imageops::FilterType::Lanczos3
//     );
//     //resized.save("mario-ouput.png")?;

//     Ok(resized)
// }


//use std::io::Cursor;
//use image::DynamicImage;

//slice: &[u8]
// #[wasm_bindgen]
// pub fn img_resize(mut source_img: &[u8]) -> Result<(), JsValue> {
//   //let img = ImageReader::open("./src/mario.png")?.decode()?;
//   let img_slice = *(source_img).to_vec();
//   let img_vec = img_slice.clone();
//   let resized = image::imageops::resize(
//     &img_vec,
//     141,
//     191,
//     image::imageops::FilterType::Lanczos3
//   );
//   //resized.save("mario-ouput.png")?;
//   Ok(())
// }

#[wasm_bindgen]
pub fn img_resize(path: &str) -> Result<String,JsError> {
    catch! {
        try {
            let img = ImageReader::open(path)?.decode()?;
            let resized = image::imageops::resize(
              &img,
              141,
              191,
              image::imageops::FilterType::Lanczos3
            );
            let vec_content = resized.into_raw();
            let slice_content = vec_content.as_slice();
            let content = String::from_utf8_lossy(slice_content);
            let response = format!("{}",content);
            
            //resized.save("mario-ouput.png")?;
            alert("entrou!!");
            Ok(response)
        }
        catch error: io::Error {
            println!("Failed to open the file: {}", error);
            let str_err = error.to_string().as_str();
            Ok(JsError::new(str_err))
        }
    };

    
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}