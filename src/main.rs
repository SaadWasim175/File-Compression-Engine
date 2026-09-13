use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;
use minifb::Key::P;

use crate::bit_writer::{BufBitWriter, Writer, WriterU16};
use crate::helper_funcs::{compress_file_huffman, decompress_file_huffman, write_compressed_data};
use crate::huffman::compress_huffman_file_write;
use crate::lzw::{buffered_lzw_compression_write_file, buffered_lzw_decompression_file_write, compress_via_lzw, decompress_lzw, non_buffered_lzw_compression_file_write};

mod helper_funcs;
mod reader;
mod jpg_to_animg;
mod metadata;
mod rle_compression;
mod huffman;
mod bit_writer;
mod lzw;

//for lzw, skip first 9 bytes since that contains the header data.
fn main() {
    // let mut data = fs::read("b.compr").unwrap();

    // data.drain(0..9);

    // let mut compr_u16: Vec<u16> = vec![];

    // let mut writer = WriterU16::new(&mut compr_u16);

    // for &byte in data.iter(){
    //     for i in (0..8).rev(){
    //         let bit = (byte >> i) & 1;
    //         writer.write_bit(bit);
    //     }
    // }

    // let mut dictionary: HashMap<u16, Vec<u8>> = HashMap::new();

    // let mut old_phrase: Vec<u8> = vec![];

    // let mut dictionary_counter = 256;

    // let decompr = decompress_lzw(&compr_u16, &mut dictionary, &mut old_phrase, &mut dictionary_counter);

    // let mut file = OpenOptions::new().read(true).write(true).create(true).open("b_non_buf_test.bmp").unwrap();

    // file.write_all(&decompr).unwrap();

    decompress_file_huffman("buf_huffman.compr");
}