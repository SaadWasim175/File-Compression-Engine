use crate::bit_writer::{BufBitWriter, Writer};
use crate::metadata;
use crate::huffman::{compress_via_huffman, decompress_via_huffman};
use std::path::Path;
use std::fs::File;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn compress_file_huffman(path: &str) {
    let file = File::open(path).unwrap();
    let raw_bytes = fs::read(path).unwrap();
    println!("Bytes read from raw file: {}", raw_bytes.len());

    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap();

    let mut to_compress = vec![]; 
    metadata::write_metadata(&mut to_compress, file.metadata().unwrap().len(), extension);

    to_compress.extend_from_slice(&raw_bytes);

    let compressed = compress_via_huffman(&mut to_compress);
    println!("Compressed file layout size: {} bytes", compressed.len());

    let name_part = path
        .split_once('.')
        .map(|(prefix, _suffix)| prefix)
        .unwrap_or(path);

    let new_file = name_part.to_owned() + ".compr";

    let mut comp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(new_file)
        .unwrap();

    comp_file.write_all(&compressed).unwrap();
    println!("Compression Successful.");
}

pub fn decompress_file_huffman(path: &str) {
    let extension = Path::new(path)
        .extension()
        .and_then(| ext| ext.to_str())
        .unwrap();

    if extension != "compr" {
        println!("Inputted file is not a compressed file. Give a compressed file please.");
        return;
    }

    let mut bytes = fs::read(path).unwrap();

    
    let mut header = [0u8; 24];
    
    for i in 0..24{
        header[i] = bytes[i];
    }

    let name_part = path
        .split_once('.')
        .map(|(prefix, _suffix)| prefix)
        .unwrap_or(path);

    let format = metadata::read_format(&mut header);

    println!("Original format was: {format}");

    let mut decom_vec = decompress_via_huffman(&mut bytes);

    let comb_string: String = format!("{}.{}", name_part, format);
    
    let return_val: &str = &comb_string;

    let mut decomp_file = OpenOptions::new().write(true).read(true).create(true).open(return_val).unwrap();

    decom_vec.drain(0..24);

    decomp_file.write_all(&decom_vec).unwrap();

}

pub fn write_compressed_data(compr: Vec<u16>, writer: &mut BufBitWriter<&mut File>) {
    for val in compr.iter() {
        for i in (0..12).rev() {
            let bit = ((val >> i) & 1) as u8;
            writer.write_bit(bit);
        }
    }
}

pub fn read_lzw_compr_file(file: &mut File) {
    
}