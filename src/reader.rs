use std::{fs::File, time::Instant};
use std::io::{Read, Seek};
use minifb::{Window, WindowOptions, self, Key};

pub fn an_img_reader(file: &mut Option<File>, pixel_buffer: Vec<u8>) {

    match file {
        Some(file) => {
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            let t = Instant::now();
            let mut verification_byte = [0u8; 5];
            file.read_exact(&mut verification_byte).unwrap();

            if verification_byte != "animg".as_bytes() {
                println!("Wrong file type.");
                return;
            }

            let mut metadata_buffer: [u8; 4] = [0u8; 4];
            file.read_exact(&mut metadata_buffer).unwrap();

            println!("{} and {} w", metadata_buffer[0], metadata_buffer[1]);
            println!("{} and {} h", metadata_buffer[2], metadata_buffer[3]);

            let width: u16 = u16::from_le_bytes([metadata_buffer[1], metadata_buffer[0]]);
            let height: u16 = u16::from_le_bytes([metadata_buffer[3], metadata_buffer[2]]);

            println!("width: {width}");
            println!("height: {height}");

            let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];

            let mut i = 0;

            let mut raw: Vec<u8> = vec![0u8; width as usize * height as usize * 3];
            file.read_exact(&mut raw).unwrap();

            let mut c = 0;

            while c < raw.len() {
                let r = raw[c] as u32;
                let g = raw[c + 1] as u32;
                let b = raw[c + 2] as u32;
                
                let pixel = (r << 16) | (g << 8) | b;
                buffer[i] = pixel;

                i += 1;
                c += 3;
            }

            
            println!("time taken to allocate to file: {:#?}", t.elapsed());
            let mut window = Window::new("Image", width as usize, height as usize, WindowOptions { resize: true, scale_mode: minifb::ScaleMode::Stretch, ..WindowOptions::default() }).unwrap();
            
            let (mut lw, mut lh) = window.get_size();

            window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
            
            while window.is_open() && !window.is_key_down(Key::Escape) {
                window.update();

                if (lw, lh) != window.get_size() {
                    window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
                }

                (lw, lh) = window.get_size();
            }
        },
        None => {
            let mut verify = [0u8; 5];
            for i in 0..5 {
                verify[i] = pixel_buffer[i];
            }

            if verify != "animg".as_bytes() {
                println!("Invalid buffer passed.");
                return;
            }

            let width = u16::from_le_bytes([pixel_buffer[6], pixel_buffer[5]]);
            let height = u16::from_le_bytes([pixel_buffer[8], pixel_buffer[7]]);

            println!("Height: {height}");
            println!("Width: {width}");

            let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];

            let mut n = 9;
            let mut buffer_index = 0;
            
            while n < pixel_buffer.len() {
                let r = pixel_buffer[n] as u32;
                let g = pixel_buffer[n+1] as u32;
                let b = pixel_buffer[n+2] as u32;
            
                let pixel = (r << 16) | (g << 8) | b;
                buffer[buffer_index] = pixel;

                n += 3;
                buffer_index += 1;
            }

            let mut window = Window::new("Image", width as usize, height as usize, WindowOptions { resize: true, scale_mode: minifb::ScaleMode::Stretch, ..WindowOptions::default() }).unwrap();

            let (mut lw, mut lh) = window.get_size();

            window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
            
            while window.is_open() && !window.is_key_down(Key::Escape) {
                window.update();

                if (lw, lh) != window.get_size() {
                    window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
                }

                (lw, lh) = window.get_size();
            }
        }
    }

}