use std::{fs::{File, OpenOptions}, io::{Seek, Write}, os::unix::fs::MetadataExt};
use crate::{bit_writer::{self, BufBitWriter}, metadata::{HEADER_LEN, write_metadata}};
use std::io::Read;
use bit_writer::WritesBits;
use crate::Writer;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Option<u8>
}

#[derive(Clone, Copy, Debug)]
pub struct Code {
    bits: u32,
    len: u8
}

impl Code {
    fn write_bit(&mut self, bit: u8) {
        self.bits = (self.bits << 1) | bit as u32;
        self.len += 1;
    }
}

impl Node {
    fn new(c: Option<u8>) -> Node {
        Node{ freq: 1, left: None, right: None, val: c }
    }
    fn from(c: Option<u8>, freq: u32) -> Node {
        Node{ freq, left: None, right: None, val: c}
    }
}

pub fn traverse(node: &Option<Box<Node>>, table: &mut Vec<Code>, path: &mut Code) {
    if let Some(current_node) = node {
        if let Some(val) = current_node.val {
            table[val as usize] = path.clone();
        }
        if current_node.left.is_some() {
            let mut left_path = path.clone();
            left_path.write_bit(0);
            traverse(&current_node.left, table, &mut left_path);
        }
        if current_node.right.is_some() {
            let mut right_path = path.clone();
            right_path.write_bit(1);
            traverse(&current_node.right, table, &mut right_path);
        }
    }
}

pub fn get_frequency(byte_vec: &mut [u8]) -> [u32; 256] {
    let mut index = 0;
    let mut freq = [0u32; 256];

    while index < byte_vec.len() {
        freq[byte_vec[index] as usize] += 1;

        index += 1;
    }
            
    freq
}

pub fn write_tree(bytes_vec: &mut [u8]) -> Node {

    let freq = get_frequency(bytes_vec);

    let mut freq_u8 = [0u8; 1024];

    let mut i = 0;

    for bytes in freq {
        let byte = u32::to_le_bytes(bytes);
        freq_u8[i] = byte[0];
        freq_u8[i+1] = byte[1];
        freq_u8[i+2] = byte[2];
        freq_u8[i+3] = byte[3];
        i += 4;
    }

    let mut freqs: Vec<Node> = vec![];

    for byte in 0..256{
        if freq[byte] > 0 {
            freqs.push(Node::from(Some(byte as u8), freq[byte]));
        }
    }

    while freqs.len() > 1 {
        freqs.sort_by(|a, b| {
            a.freq
            .cmp(&b.freq)
            .then_with(|| a.val.cmp(&b.val))
        });

        let smallest = freqs.remove(0);
        let second_smallest = freqs.remove(0);

        let parent = Node{
            freq: smallest.freq as u32 + second_smallest.freq as u32,
            left: Some(Box::new(smallest)),
            right: Some(Box::new(second_smallest)),
            val: None
        };

        freqs.push(parent);
    }
    
    freqs[0].clone()
}

fn encode<W:Write>(table: &mut Vec<Code>, writer: &mut impl WritesBits<W>, bytes: &mut [u8]) {
    for byte in bytes.iter() {
        let code = &table[*byte as usize];

        for i in (0..code.len).rev(){
            let bit = ((code.bits >> i) & 1) as u8;
            writer.write_bit(bit);
        }
    }
}

pub fn compress_huffman_file_write(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut buf = [0u8; 6*1000*1000];
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let init_bytes = file.read(&mut buf).unwrap();

    // println!("First 11 bytes are: {:#?}", buf[..11].iter());

    let mut output_file = OpenOptions::new().create(true).write(true).read(true).truncate(true).open("buf_huffman.compr").unwrap();

    let tree = write_tree(&mut buf[..init_bytes]);
    let mut table = vec![Code{bits: 0, len: 0}; 256];
    let freq = get_frequency(&mut buf[..init_bytes]);
    let mut path = Code{bits: 0, len: 0};
    traverse(&Some(Box::new(tree)), &mut table, &mut path);

    let mut freq_u8 = [0u8; 1024];
    let mut c = 0;

    for i in 0..256 {
        let u8_values = u32::to_le_bytes(freq[i]);
        freq_u8[c] = u8_values[0];
        freq_u8[c+1] = u8_values[1];
        freq_u8[c+2] = u8_values[2];
        freq_u8[c+3] = u8_values[3];
        c += 4;
    }

    
    let mut headers = vec![];
    write_metadata(&mut headers, file.metadata().unwrap().size(), "txt");

    output_file.write_all(&headers).unwrap();
    output_file.write_all(&freq_u8).unwrap();

    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let mut writer = BufBitWriter::new(0, output_file);

    loop {
        let bytes = file.read(&mut buf).unwrap();
        let first_bytes = &buf[..11];

        println!("firsdt 11 bytes are: ");
        for byte in first_bytes {
            println!("{}", byte.to_string())
        }


        if bytes == 0 {
            break;
        }

        encode(&mut table, &mut writer, &mut buf[0..bytes]);
    }
    writer.flush();

}

#[deprecated]
pub fn compress_via_huffman(bytes: &mut [u8]) -> Vec<u8> {
    let tree = write_tree(bytes);
    let mut table = vec![Code{bits: 0, len: 0}; 256];
    let freq = get_frequency(bytes);
    let mut path = Code{bits: 0, len: 0};
    traverse(&Some(Box::new(tree)), &mut table, &mut path);
    
    let mut freq_u8 = [0u8; 1024];

    let mut c = 0;

    for i in 0..256{
        let u8_values = u32::to_le_bytes(freq[i]);
        freq_u8[c] = u8_values[0];
        freq_u8[c+1] = u8_values[1];
        freq_u8[c+2] = u8_values[2];
        freq_u8[c+3] = u8_values[3];
        c += 4
    }

    let mut header = [0u8; HEADER_LEN];

    for i in 0..HEADER_LEN{
        header[i] = bytes[i];
    }

    for i in 0..256 {
        if table[i].len > 0 {
            println!(
                "{} -> {:b} ({})",
                i,
                table[i].bits,
                table[i].len
            )
        }
    }
    let mut output_vec: Vec<u8> = vec![];

    output_vec.write_all(&header).unwrap();
    output_vec.write_all(&freq_u8).unwrap();

    let mut writer = Writer::new(0, &mut output_vec);
    encode(&mut table, &mut writer, bytes);
    writer.flush();

    output_vec

}

fn read_freq(bytes: &mut [u8]) -> [u8; 1024] {
    let mut freqs_u8 = [0u8; 1024];
    for (ind, i) in bytes[HEADER_LEN..HEADER_LEN + 1024].iter().enumerate(){
        freqs_u8[ind] = *i;
    }
    freqs_u8
}

fn get_headers(bytes: &mut [u8]) -> [u8; HEADER_LEN] {
    let mut header = [0u8; HEADER_LEN];
    for i in 0..HEADER_LEN{
        header[i] = bytes[i];
    }
    header
}

fn generate_tree(freq: [u8; 1024]) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    let mut i: usize = 0;

    for byte_counter in 0u8..=255 {
        let freq_as_u32 = u32::from_le_bytes([freq[i], freq[i+1], freq[i+2], freq[i+3]]);
        if freq_as_u32 > 0 {
            nodes.push(Node::from(Some(byte_counter), freq_as_u32));
        }
        i += 4;
    }
    
    while nodes.len() > 1 {
        nodes.sort_by(|a, b| {
            a.freq
                .cmp(&b.freq)
                .then_with(|| a.val.cmp(&b.val))
        });
        let smallest = nodes.remove(0);
        let second_smallest = nodes.remove(0);

        let the_parent = Node {
            freq: smallest.freq as u32 + second_smallest.freq as u32,
            left: Some(Box::new(smallest)),
            right: Some(Box::new(second_smallest)),
            val: None
        };
        nodes.push(the_parent);
    }
    nodes
}

pub fn decompress_via_huffman(bytes: &mut [u8]) -> Vec<u8> {
    let header = get_headers(bytes);
    let freqs = read_freq(bytes);
    let tree = generate_tree(freqs);
    
    let mut index = HEADER_LEN + 1024 as usize;

    let mut buffer = [0u8; 9000];
    let root = &tree[0];
    let mut current_node = root;
    let mut resultant_buffer: Vec<u8> = vec![];

    let bytes_in_resultant_buffer = u64::from_le_bytes([header[5], header[6], header[7], header[8], header[9], header[10], header[11], header[12]]);

    println!(" bytes in resultant buffer = {bytes_in_resultant_buffer}");

    while resultant_buffer.len() < bytes_in_resultant_buffer as usize {
        let bytes = bytes[index..].iter().as_slice().read(&mut buffer).unwrap();
        if bytes == 0 {
            break;
        }

        for byte in buffer[..bytes].iter() {
            for i in (0..u8::BITS).rev() {
                if resultant_buffer.len() >= bytes_in_resultant_buffer as usize {
                    break;
                }

                let bit = (byte >> i) & 1;
                if bit == 0 {
                    current_node = current_node.left.as_ref().unwrap();
                } else if bit == 1 {
                    current_node = current_node.right.as_ref().unwrap();
                }

                if let Some(val) = current_node.val {
                    resultant_buffer.push(val);
                    current_node = root;
                }
            }
        }
        index += bytes;
    }
    resultant_buffer
}
