use std::{fs::File, io::Write};

//right now, follows structure:
//identifier/magic number: 5 bytes (CONSTANT, its always compr)
//original size: 8 bytes
//file format length: 1 bytes
//file format: 10 bytes

pub const HEADER_LEN: usize = 5 + 8 + 1 + 10;

pub fn write_metadata(vector: &mut Vec<u8>, og_size: u64, format: &str) {
    let unique_ident = "compr";
    let ident_bytes = unique_ident.as_bytes();
    let format_bytes = format.as_bytes();

    let og_bytes = u64::to_le_bytes(og_size);

    vector.write_all(ident_bytes).unwrap();
    vector.write_all(&og_bytes).unwrap();
    vector.write_all(&[format_bytes.len() as u8]).unwrap();
    vector.write_all(format_bytes).unwrap();
    if format_bytes.len() < 10 {
        let ending_zeros_len = 10 - format_bytes.len();

        let ending_zeros = vec![0u8; ending_zeros_len];
        vector.write_all(&ending_zeros).unwrap(); 
    }
}

pub fn read_format(header: &mut [u8; 24]) -> String {
    let format_len = header[13];

    let format_itself = &header[14..24];

    let mut array_to_read = vec![0u8; format_len as usize];

    for i in 0..format_len{
        array_to_read[i as usize] = format_itself[i as usize];
    }

    String::from_utf8_lossy(&array_to_read).into_owned()
}

//og file format
//note: please only pass empty output vectors to this function.
//Note: using 10 bytes for the format storage.
pub fn lzw_metadata(format: &str, file: &mut File) {
    
    file.write_all(format.as_bytes()).unwrap();
    let mut vec_zeroes = vec![0u8; (10 - format.len())-1];
    
    file.write_all(&vec_zeroes).unwrap();
}