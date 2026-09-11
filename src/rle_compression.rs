use std::{fs::File, io::{Read, Write}};

use minifb::{Window, WindowOptions, Key};

use crate::metadata::HEADER_LEN;

//compresses in the following manner:
// [R] [G] [B] [A]
// A is the amount of times repeated
// NOTE: A IS 2 BYTES LONG

//NOTE: NEEDS TO BE MODIFIED TO WORK WITH INDIVIDUAL BYTES
pub fn compress_via_rle(file: &mut File) -> Vec<u8> {

    let mut buf = [0u8; 9000];
    let mut header = [0u8; HEADER_LEN];
    
    file.read_exact(&mut header).unwrap();
    let mut file2 = File::create("rle.an_img").unwrap();
    file2.write_all(&mut header).unwrap();

    let mut bytes_vector: Vec<u8> = vec![];

    bytes_vector.write_all(&header).unwrap();

    loop {
        let bytes = file.read(&mut buf).unwrap();
        
        if bytes == 0{
            break;
        }

        let mut i = 0;
        let mut arr: Vec<[u8; 3]> = vec![];

        while i + 2 < bytes {
            arr.push([buf[i], buf[i+1], buf[i+2]]);
            i += 3;
        }

        let chunks: Vec<&[[u8; 3]]> = arr.chunk_by(|a, b| a == b).collect();

        let chunk_len = chunks.len();

        if chunk_len >= 65536 {
            
        }

        for val in chunks {
            let len = val.len() as u16;
            let len_u8 = len.to_le_bytes();

            file2.write_all(&val[0]).unwrap();
            
            bytes_vector.push(val[0][0]);
            bytes_vector.push(val[0][1]);
            bytes_vector.push(val[0][2]);
            
            file2.write_all(&len_u8).unwrap();
        
            bytes_vector.push(len_u8[0]);
            bytes_vector.push(len_u8[1]);
        }


    }

    bytes_vector
}

pub fn decompress_rle(bytes: Vec<u8>) -> Vec<u8> {
    let mut header = [0u8; 9];

    for i in 0..9{
        header[i] = bytes[i];
    }

    if header[..5] != *"animg".as_bytes() {
        println!("Invalid file not an_img (rle_compression error).");
        return vec![];
    }

    let width = u16::from_le_bytes([header[6], header[5]]);
    let height = u16::from_le_bytes([header[8], header[7]]);

    println!("Width: {width}, height: {height}");
    
    let mut index = 9;

    let mut return_vec: Vec<u8> = Vec::with_capacity(9 + (width as usize * height as usize * 3));
    return_vec.write_all(&header).unwrap();

    while index + 4 < bytes.len() {
        let r = bytes[index];
        let g = bytes[index + 1];
        let b = bytes[index + 2];

        let repeat1 = bytes[index + 3];
        let repeat2 = bytes[index + 4];

        let repeat = u16::from_le_bytes([repeat1, repeat2]);

        for _ in 0..repeat{
            return_vec.push(r);
            return_vec.push(g);
            return_vec.push(b);
        }

        index += 5;
    }

    return_vec
}

pub fn read_rle(file: &mut File) {
    let mut buf = [0u8; 9000];
    let mut header = [0u8; 9];
    
    file.read_exact(&mut header).unwrap();

    if header[..5] != *"animg".as_bytes() {
        println!("Invalid an_img file");
        return;
    }

    let width: u16 = u16::from_le_bytes([header[6], header[5]]);
    let height: u16 = u16::from_le_bytes([header[8], header[7]]);

    println!("width: {width}, height: {height}");

    let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];

    let mut out = 0;
    loop {
        let bytes = file.read(&mut buf).unwrap();
        
        if bytes == 0 {
            break;
        }
        let mut index = 0;
        while index + 4 < bytes{
            let r = buf[index] as u32;
            let g = buf[index + 1] as u32;
            let b = buf[index + 2] as u32;

            let repeat1 = buf[index + 3];
            let repeat2 = buf[index + 4];

            let pixel = (r << 16) | (g << 8) | b;

            let repeat = u16::from_le_bytes([repeat1, repeat2]);
            index += 5;
            for _ in 0..repeat {
                buffer[out] = pixel;
                out += 1;
            }

        }
        
    }

    let mut window = Window::new("Image", width as usize, height as usize, WindowOptions{resize: true, scale_mode: minifb::ScaleMode::Stretch, ..WindowOptions::default()}).unwrap();

    let (lw, lh) = window.get_size();
    window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();

        if (lw, lh) != window.get_size() {
            window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
        }
    }
}