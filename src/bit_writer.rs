use core::panic;
use std::io::Write;

pub trait WritesBits<W:Write> {
    fn new(byte: u8, writer: W) -> Self;
    fn write_bit(&mut self, bit: u8);
    fn flush(&mut self);
}

pub struct Writer<W: Write> {
    writer: W,
    byte: u8,
    bits_filled: u8,
}

pub struct BufBitWriter<W: Write> {
    writer: W, 
    byte: u8, 
    bits_filled: u8,
    buffer: Vec<u8>,
    buffer_filled: usize
}

impl<W: Write> WritesBits<W> for BufBitWriter<W>  {
    fn new(byte: u8, writer: W) -> BufBitWriter<W> {
        let mut buffer = vec![0u8; 6*1000*1000];
        let mut buffer_filled = 0;
        BufBitWriter { writer, byte, bits_filled: 0, buffer, buffer_filled }
    }

    fn write_bit(&mut self, bit: u8) {
        if bit != 0 && bit != 1 {
            panic!("Value passed is not a bit.");
        } else {
            self.byte = (self.byte << 1) | bit;
            self.bits_filled = self.bits_filled + 1;
        }
        if self.bits_filled >= 8 {
            self.buffer[self.buffer_filled] = self.byte;
            self.buffer_filled += 1;
            self.byte = 0;
            self.bits_filled = 0;
        }
        if self.buffer_filled >= self.buffer.len() {
            self.writer.write_all(&self.buffer).unwrap();
            self.buffer = vec![0u8; 8*1000*1000];
            self.buffer_filled = 0;
        }
    }

    fn flush(&mut self) {
        if self.bits_filled > 0 {
            self.byte <<= 8 - self.bits_filled;
            self.buffer[self.buffer_filled] = self.byte;
            self.buffer_filled += 1;

            self.byte = 0;
            self.bits_filled = 0;
        }

        if self.buffer_filled > 0 {
            self.writer.write_all(&self.buffer[..self.buffer_filled]).unwrap();
            self.buffer_filled = 0;
        }
    }
}

impl<W: Write> WritesBits<W> for Writer<W> {
    fn new(byte: u8, writer: W) -> Writer<W> {
        Writer { writer, byte, bits_filled: 0 }
    }

    fn write_bit(&mut self, bit: u8) {
        if bit != 0 && bit != 1 {
            panic!("Value passed is not a bit.");
        } else {
            self.byte = (self.byte << 1) | bit;
            self.bits_filled = self.bits_filled + 1;
        }
        if self.bits_filled >= 8 {
            self.writer.write_all(&[self.byte]).unwrap();
            self.byte = 0;
            self.bits_filled = 0;
        }
    }

    fn flush(&mut self) {
        if self.bits_filled > 0 {
            self.byte <<= 8 - self.bits_filled;
            self.writer.write_all(&[self.byte]).unwrap();

            self.byte = 0;
            self.bits_filled = 0;
        }
    }
}

pub struct WriterU16<'a>{
    writer: &'a mut Vec<u16>,
    bytes_2: u16,
    bits_filled: u8,
}

impl WriterU16<'_> {
    pub fn new(dest: &mut Vec<u16>) -> WriterU16 {
        WriterU16 { writer: dest, bytes_2: 0, bits_filled: 0 }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if bit != 0 && bit != 1 {
            panic!("Value passed is not a bit.");
        } else {
            self.bytes_2 = (self.bytes_2 << 1) | bit as u16;
            self.bits_filled += 1;
        }
        if self.bits_filled >= 12 {
            self.writer.push(self.bytes_2);
            self.bits_filled = 0;
            self.bytes_2 = 0;
        }
    }

    pub fn flush(&mut self) {
        if self.bits_filled > 0 {
            self.bytes_2 <<= 8 - self.bits_filled;
            self.writer.push(self.bytes_2);
            self.bytes_2 = 0;
            self.bits_filled =  0;
        }
    }
}