//first 256 entries correspond to the value being compressed, i.e 0b0 is 0, 0b1 is 1 and so forth
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::time::Instant;
use crate::bit_writer::BufBitWriter;
use crate::bit_writer::WriterU16;
use crate::metadata::lzw_metadata;
use crate::write_compressed_data;
//dictionary needs to be initialized with values 0-255 first
//dictionary_count needs to be equal to 256
//

//always make sure hashmap_counter is set to 256 before passing it in
//current should also be set to bytes[0]
//dictioary must always be empty

pub fn compress_via_lzw(bytes: &[u8], dictionary: &mut HashMap<(u16, u8), u16>, hashmap_counter: &mut u16, current: &mut u16) -> Vec<u16> {
    let mut output_vec:Vec<u16> = vec![];

    for &byte in bytes.iter() {
        if let Some(val) = dictionary.get(&(*current, byte)) {
            *current = *val;
        } else {
            output_vec.push(*current);

            if *hashmap_counter < 4096 {
                dictionary.insert((*current, byte), *hashmap_counter);
                
                *hashmap_counter += 1;
            }
            *current = byte as u16;
        }
    }
    
    output_vec
}

//buffered lzw compression is fixed
pub fn buffered_lzw_compression_write_file(file_path: &str) {
    let duration = Instant::now();

    let mut file = File::open(file_path).unwrap();
    let file_name = file_path.split_once('.').unwrap();
    let compr_name = format!("{}_buf.compr", file_name.0);
    let mut file2 = OpenOptions::new().write(true).read(true).create(true).open(&compr_name).unwrap();

    lzw_metadata(file_name.1, &mut file2);

    let mut dictionary: HashMap<(u16, u8), u16> = HashMap::new();
    let mut dict_count = 256;
    let mut writer = BufBitWriter::new(0, &mut file2);

    let mut buf = [0u8; 6*1000*1000];
    let mut is_first_loop = true;

    let mut current: u16 = 0;
    let mut compr: Vec<u16>;

    loop {
        let bytes = file.read(&mut buf).unwrap();
        
        if is_first_loop {
            current = buf[0] as u16;
            compr = compress_via_lzw(&buf[1..bytes], &mut dictionary, &mut dict_count, &mut current);
        } else {
            compr = compress_via_lzw(&buf[..bytes], &mut dictionary, &mut dict_count, &mut current);
        }

        if bytes == 0 {
            break;
        }


        write_compressed_data(compr, &mut writer);

        is_first_loop = false;
    }
    write_compressed_data(vec![current], &mut writer);
    writer.flush();
    let time = duration.elapsed();
    println!("Time taken for buffered compression: {}s", time.as_secs());
}

pub fn non_buffered_lzw_compression_file_write(file_path: &str) {
    let duration = Instant::now();
    let data = fs::read(file_path).unwrap();
    
    let file_name = file_path.split_once('.').unwrap();
    let compr_name = format!("{}.compr", file_name.0);
    let mut file2 = OpenOptions::new().create(true).read(true).write(true).open(compr_name).unwrap();
    lzw_metadata(file_name.1, &mut file2);

    let mut dictionary: HashMap<(u16, u8), u16> = HashMap::new();
    let mut dictionary_count = 256;

    let mut current = data[0] as u16;

    let mut writer = BufBitWriter::new(0, &mut file2);

    let mut compr = compress_via_lzw(&data[1..], &mut dictionary, &mut dictionary_count, &mut current);

    compr.push(current);

    write_compressed_data(compr, &mut writer);

    let time = duration.elapsed();
    println!("Time taken for non buffered compression: {}s", time.as_secs());
    writer.flush();

}

pub fn buffered_lzw_decompression_file_write(file_path: &str) {
    let mut file = File::open(file_path).unwrap();
    let file_name_ext = file_path.split_once('.').unwrap();
    let output_file_name = format!("{}_buf_decomp.bmp", file_name_ext.0);

    file.seek(std::io::SeekFrom::Start(9)).unwrap();
    let mut buf = [0u8; 6*1000*1000];
    let mut old_phrase:Vec<u8> = vec![];
    let mut dictionary: HashMap<u16, Vec<u8>> = HashMap::new();
    let mut dict_counter = 256;
    
    let mut output_file = OpenOptions::new().read(true).write(true).create(true).open(output_file_name).unwrap();
    
    loop {
        let bytes = file.read(&mut buf).unwrap();
        
        let mut compr_u16 = vec![];
        if bytes == 0 {
            break;
        }
        {
            let mut writer = WriterU16::new(&mut compr_u16);
            for byte in buf[..bytes].iter() {
                for i in (0..8).rev() {
                    let bit = (byte >> i) & 1;
                    writer.write_bit(bit);
                }
            }
        }
        let decompr = decompress_lzw(&compr_u16, &mut dictionary, &mut old_phrase, &mut dict_counter);

        output_file.write_all(&decompr).unwrap();
    }
}

pub fn decompress_lzw(codes: &[u16], dictionary: &mut HashMap<u16, Vec<u8>>, old_phrase: &mut Vec<u8>, dictionary_counter: &mut u16) -> Vec<u8> {
    let mut output_vec: Vec<u8> = vec![];

    for &code in codes.iter() {
        let current_phrase: Vec<u8>;

        if code < 256 {
            current_phrase = vec![code as u8];
        } else if let Some(bytes) = dictionary.get(&code) {
            current_phrase = bytes.clone();
        } else if code == *dictionary_counter && !old_phrase.is_empty() {
            let mut kwk_phrase = old_phrase.clone();
            kwk_phrase.push(old_phrase[0]);
            current_phrase = kwk_phrase;
        } else {
            panic!("Corrupted LZW stream: code {} is unresolvable at dict index {}", code, dictionary_counter);
        }

        output_vec.extend_from_slice(&current_phrase);

        if !old_phrase.is_empty() && *dictionary_counter < 4096 {
            let mut new_entry = old_phrase.clone();
            new_entry.push(current_phrase[0]);

            dictionary.insert(*dictionary_counter, new_entry);
            *dictionary_counter += 1;    
        }

        *old_phrase = current_phrase;
    }
    
    output_vec    
}

