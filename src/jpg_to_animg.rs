// use std::{fs::File, io::Write};

// use crate::metadata::write_metadata;

// pub fn jpg_to_animg(path: &str) -> Vec<u8> {

//     let mut bytes_vector: Vec<u8> = vec![];

//     let img = match image::open(path){
//         Ok(img) => img, 
//         Err(e) => {
//             panic!("error encountered: {e:#?}")
//         }
//     };

//     let rgb = img.to_rgb8();

//     let (width, height) = rgb.dimensions();

//     let width_hb = ((width >> 8) & 0xFF) as u8;
//     let width_lb = (width & 0xFF) as u8;

//     let height_hb = ((height >> 8) & 0xFF) as u8;
//     let height_lb = (height & 0xFF) as u8;
    
//     println!("converted width is {:?}", [width_hb, width_lb]);
//     println!("converted height is {:?}", [height_hb, height_lb]);

//     let mut file = File::create_new("test.an_img").unwrap();
//     write_metadata(&mut bytes_vector, &[width_hb, width_lb], &[height_hb, height_lb]);

//     file.write_all(rgb.as_raw()).unwrap();

//     vec![]
// }