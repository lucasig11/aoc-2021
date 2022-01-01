#[derive(Copy, Clone, Debug)]
pub struct BitStream<'a> {
    data: &'a [u8],
    current_byte: usize,
    current_bit: u8,
    byte_len: u8,
}

impl<'a> BitStream<'a> {
    pub fn new(data: &'a [u8], byte_len: u8) -> BitStream {
        BitStream {
            data,
            current_byte: 0,
            current_bit: 0,
            byte_len,
        }
    }

    pub fn advance_by(&mut self, n: u8) -> Option<u64> {
        let mut result = 0;
        for _ in 0..n {
            result <<= 1;
            result |= self.advance()? as u64;
        }
        Some(result)
    }

    pub fn advance(&mut self) -> Option<u8> {
        if self.current_byte >= self.data.len() {
            return None;
        }
        let byte = self.data[self.current_byte];
        let bit = (byte >> ((self.byte_len - 1) - self.current_bit)) & 1;
        self.current_bit += 1;
        if self.current_bit == self.byte_len {
            self.current_bit = 0;
            self.current_byte += 1;
        }
        Some(bit)
    }

    pub fn bits_read(&self) -> u64 {
        (self.current_byte as u64) * (self.byte_len as u64) + (self.current_bit as u64)
    }
}

impl std::fmt::Binary for BitStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.current_byte..self.data.len() {
            write!(f, "{:0d$b} ", self.data[i], d = self.byte_len as usize)?;
        }
        writeln!(f)
    }
}
