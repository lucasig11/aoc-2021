#[derive(Copy, Clone, Debug)]
pub struct BitStream {
    pub data: &'a [u8],
    pub current_byte: usize,
    pub current_bit: u8,
    pub byte_len: u8,
}

impl BitStream {
    pub fn new(data: &'a [u8], byte_len: u8) -> BitStream {
        BitStream {
            data,
            current_byte: 0,
            current_bit: 0,
            byte_len,
        }
}

impl Iterator for BitStream {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.current_bit == 0 {
            self.current_bit = 8;
            self.current_byte += 1;
        }

        if self.current_byte == self.byte_len {
            return None;
        }

        let bit = (self.data[self.current_byte] & (1 << (self.current_bit - 1))) != 0;
        self.current_bit -= 1;

        Some(bit)
    }
}

impl std::ops::Index<usize> for BitStream {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        let bit = (self.data[index / 8] & (1 << (index % 8))) != 0;
        &bit
    }
}
