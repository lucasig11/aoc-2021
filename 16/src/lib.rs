#![feature(test)]
extern crate test;

use std::{
    fmt::{Binary, Debug},
    iter::Peekable,
};

mod bistream;

use bitstream::BitStream;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<u8>;

use bitflags::bitflags;

bitflags! {
    struct PacketFlags: u8 {
        const VERSION = 0b1110_0000;
        const TYPE_ID = 0b0001_1100;
    }
}

macro_rules! dump_bin {
    ($digits: expr; $($n:expr),*) => {
        {
            $(
                print!("{:0d$b} ", $n, d=$digits);
            )*
            println!();
        }
    };
    ($($n:expr),*) => {
        {
            $(
                print!("{:04b} ", $n);
            )*
            println!();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, Clone, Copy)]
pub enum PacketTypeId {
    Literal,
    Operator(u8),
}

impl From<u8> for PacketTypeId {
    fn from(value: u8) -> Self {
        match value {
            4 => PacketTypeId::Literal,
            v => PacketTypeId::Operator(v),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PacketHeader {
    pub version: u8,
    pub type_id: PacketTypeId,
}

// 110 100 1 0111 1 1110 0 0101 000
// VVV TTT A AAAA B BBBB C CCCC
#[derive(Debug, Clone, Copy)]
pub struct LiteralPacket {
    pub header: PacketHeader,
    pub literal: u64,
}

// 001 110 0 000000000011011 11010001010 0101001000100100 0000000
// VVV TTT I LLLLLLLLLLLLLLL AAAAAAAAAAA BBBBBBBBBBBBBBBB
#[derive(Debug, Clone)]
pub struct OperatorPacket {
    pub header: PacketHeader,
    pub sub_packets: Vec<Packet>,
}

// Length from most to least significant bit
fn byte_len(mut n: u8) -> usize {
    let mut cnt = 0;
    while n != 0 {
        n >>= 1;
        cnt += 1;
    }
    cnt.max(4)
}

pub fn get_bit(n: &[u8], sz: usize, bit: usize) -> u8 {
    let byte = bit / 4;
    let bit = bit % 4;
    if byte < sz {
        return (n[byte] >> (3 - bit)) & 1;
    }
    panic!("get_bit overflow");
}

pub fn get_bits(n: &[u8], sz: usize, start: &mut usize, end: usize) -> u64 {
    let mut res = 0;
    for i in *start..(*start + end) {
        res <<= 1;
        res |= get_bit(n, sz, i) as u64;
    }
    *start += end;
    res
}

pub fn parse_packet(n: &[u8]) {
    // Current byte index;
    let mut stream = BitStream::new(n, 4);

    let mut pc = 0;
    let sz = n.len();
    let version = get_bits(n, sz, &mut pc, 3) as u8;
    let type_id = get_bits(n, sz, &mut pc, 3) as u8;

    println!("Version: {}", version);
    println!("Type ID: {}", type_id);

    let type_id: PacketTypeId = type_id.into();

    let header = PacketHeader { version, type_id };

    match type_id {
        PacketTypeId::Literal => {}
        PacketTypeId::Operator(_) => {
            let len_tid = get_bit(n, sz, pc + 1);
            println!("Len TID: {}", len_tid);
        }
    }
}

pub fn read_n_bits<T>(byte_stream: &mut Peekable<T>, n: usize) -> (u64, Option<u8>)
where
    T: Iterator<Item = u8> + Debug,
{
    assert!(n != 0);
    let mut bit_buffer = 0u64;
    let mut remaining = None;
    let num = byte_stream.next().unwrap();

    // If `num` does not have enough bits, get the missing bits from the next byte.
    let mut num = if n > byte_len(num) {
        let missing_bits = n - byte_len(num);
        let next_byte = byte_stream.peek_mut().unwrap();
        let next_bits = *next_byte >> (4 - missing_bits);
        *next_byte &= !(0b1111 << (4 - missing_bits));
        (num << missing_bits) | next_bits
    } else {
        num
    };

    // Truncate num to `n` bits.
    if byte_len(num) > n {
        let n = byte_len(num) - n;
        let mask = (1 << n) - 1;
        match remaining.as_mut() {
            Some(leftover) => *leftover |= num & mask,
            None => remaining = Some(num & mask),
        }
        num >>= n;
    }

    bit_buffer |= num as u64;

    (bit_buffer, remaining)
}

impl Packet {
    pub fn parse<T>(byte_stream: &mut Peekable<T>) -> Option<Self>
    where
        T: Iterator<Item = u8> + Debug,
    {
        // First 8 bits
        let head = byte_stream.next()? << 4 | byte_stream.next()?;
        let version = (head & PacketFlags::VERSION.bits) >> 5;
        let type_id = (head & PacketFlags::TYPE_ID.bits) >> 2;
        // 7th and 8th bits
        let remainder = head & 0b11;
        let type_id = match type_id {
            4 => PacketTypeId::Literal,
            _ => PacketTypeId::Operator(type_id),
        };
        let header = PacketHeader { version, type_id };

        println!("\n--------------------------------\n");
        dump_bin!(8; head);
        println!("Version: {}\nType ID: {:?}", version, type_id);

        match type_id {
            PacketTypeId::Literal => {
                // Set the remainder back to the first byte in the stream.
                *byte_stream.peek_mut().unwrap() |= remainder << 4;
                let mut literal = 0u64;
                let mut keep;
                let mut unused = 2;
                loop {
                    let (mut num, unused_bits) = read_n_bits(byte_stream, 5 - unused);
                    println!("Literal: {:b}", num);
                    if let Some(leftover) = unused_bits {
                        // Set the unused_bits back to the first byte in the stream.
                        *byte_stream.peek_mut().unwrap() |= leftover << 4;
                        unused = byte_len(leftover);
                    }
                    let keep_bit = 1 << 4;

                    // If the 5th bit is set, keep iterating.
                    keep = ((num & keep_bit) >> 4) as u8;
                    // Mask off the 5th bit.
                    num &= !keep_bit;

                    literal <<= 4;
                    literal |= num;
                    if keep == 0 {
                        break;
                    }
                }

                Some(Packet::Literal(LiteralPacket { header, literal }))
            }
            _ => {
                // 7th bit
                let len_tid = remainder >> 1;
                // 11 is 0b1011, 15 is 0b1111
                // If len_tid is 0 then num_sz is 15, if 1 then its 11.
                let num_sz = (len_tid | 0b1011) as usize;
                // Get the next `num_sz` bits.
                let sp_len = byte_stream
                    .take(num_sz / 4 + 1)
                    .fold(remainder as u16 & 0b1, |acc, x| (acc << 4) | (x as u16));
                let remainder = sp_len as u8 & 0b11;
                let sp_len = sp_len >> 2;
                println!("Length type ID: {}", len_tid);
                println!("Sub-packets length: {}", sp_len);

                let sub_packets = match len_tid {
                    0 => {
                        // sp_len is the total length in bits of all the subpackets in this packet
                        dump_tape(&byte_stream.collect::<Vec<_>>());
                        todo!()
                    }
                    1 => {
                        // sp_len is the total count of subpackets in this packet
                        let mut sub_packets = Vec::with_capacity(sp_len as usize);

                        // This is how the tape should look like:
                        // 0101 0000 0011 0010 0000 1000 1100 0001 1000 0000
                        let mut new = vec![remainder];
                        new.extend(&byte_stream.collect::<Vec<_>>());

                        shift_all(&mut new, 2);

                        let mut byte_stream = new.into_iter().peekable();

                        for _ in 0..sp_len {
                            let packet = Packet::parse(&mut byte_stream)?;
                            sub_packets.push(dbg!(packet));
                        }

                        sub_packets
                    }
                    _ => unreachable!(),
                };

                Some(Self::Operator(OperatorPacket {
                    header,
                    sub_packets,
                }))
            }
        }
    }
}

// shift_all [0011, 1100, 0101, 1010] 2 -> [1111, 0001, 0110, 1000]
pub fn shift_all(v: &mut [u8], offset: u8) {
    for i in 0..v.len() - 1 {
        v[i] = (v[i] << offset) | (v[i + 1] >> (4 - offset));
        v[i + 1] &= ((1 << offset) - 1) as u8;
    }
}

fn dump_tape<T: Binary>(tape: &[T]) {
    for i in tape {
        print!("{:04b} ", i);
    }
    println!();
}

fn solve_part_one(input: &ParsedInput) -> usize {
    parse_packet(input);
    let mut stream = input.iter().copied().peekable();
    let p1 = Packet::parse(&mut stream);
    println!("{:#?}", p1);

    todo!("Part One")
}

fn solve_part_two(_input: &ParsedInput) -> usize {
    todo!("Part Two")
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .trim()
        .chars()
        .fold(Vec::with_capacity(input.len()), |mut acc, el| {
            acc.push(el.to_digit(16).unwrap() as u8);
            acc
        })
}

pub fn solve() {
    // Literal packet
    // let _INPUT = "D2FE28";
    // V: 1, T: 6, I: 0, L: 27
    // let _INPUT = "38006F45291200";
    // V: 7, T: 3, I: 1, L: 3
    let _INPUT = "EE00D40C823060";
    let input = parse_input(_INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 4512);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 27027);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 1924);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 36975);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
