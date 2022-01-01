#![feature(test)]
extern crate test;

mod bitstream;

use bitstream::BitStream;
use std::fmt::Debug;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<u8>;

#[derive(Debug, Clone)]
pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, Clone, Copy)]
pub struct LiteralPacket {
    pub version: u8,
    pub literal: u64,
}

#[derive(Debug, Clone)]
pub struct OperatorPacket {
    pub version: u8,
    pub operator: u8,
    pub sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn literal(version: u8, literal: u64) -> Self {
        Packet::Literal(LiteralPacket { version, literal })
    }

    pub fn operator(version: u8, operator: u8, sub_packets: Vec<Packet>) -> Self {
        Packet::Operator(OperatorPacket {
            version,
            operator,
            sub_packets,
        })
    }

    pub fn version(&self) -> u8 {
        match self {
            Packet::Literal(p) => p.version,
            Packet::Operator(p) => p.version,
        }
    }

    pub fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal(_) => self.version() as u64,
            Packet::Operator(p) => p
                .sub_packets
                .iter()
                .fold(self.version() as u64, |acc, p| acc + p.version_sum()),
        }
    }

    fn parse(stream: &mut BitStream) -> Option<Packet> {
        let version = stream.advance_by(3)? as u8;
        let type_id = stream.advance_by(3)? as u8;

        match type_id {
            4 => {
                let mut result = 0;
                let mut keep;
                loop {
                    let mut literal = stream.advance_by(5)?;
                    let flag = 1 << 4;

                    keep = ((literal & flag) >> 4) != 0;
                    literal &= !flag;
                    result = (result << 4) | literal;

                    if !keep {
                        break;
                    }
                }

                Some(Packet::literal(version, result))
            }
            _ => {
                let len_tid = stream.advance()?;
                let num_sz = (((len_tid == 0) as u8) << 2) | 0b1011;
                let sp_len = stream.advance_by(num_sz)?;

                let sub_packets = match len_tid {
                    0 => {
                        let mut sub_packets = vec![];
                        let bits_read = stream.bits_read();
                        while stream.bits_read() - bits_read < sp_len {
                            sub_packets.push(Packet::parse(stream)?);
                        }

                        sub_packets
                    }
                    1 => (0..sp_len)
                        .map(|_| Packet::parse(stream))
                        .collect::<Option<Vec<_>>>()?,
                    _ => panic!("invalid length type id"),
                };

                Some(Packet::operator(version, type_id, sub_packets))
            }
        }
    }

    fn execute(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.literal,
            Packet::Operator(p) => {
                let op = p.operator;
                let sub_packets = p.sub_packets.iter().map(|p| p.execute());

                match op {
                    0 => sub_packets.sum(),
                    1 => sub_packets.product(),
                    2 => sub_packets.min().unwrap(),
                    3 => sub_packets.max().unwrap(),
                    5..=7 => {
                        let p = sub_packets.collect::<Vec<_>>();
                        match op {
                            5 => (p[0] > p[1]) as u64,
                            6 => (p[0] < p[1]) as u64,
                            7 => (p[0] == p[1]) as u64,
                            _ => unreachable!(),
                        }
                    }
                    _ => panic!("invalid operator"),
                }
            }
        }
    }
}

fn solve_part_one(input: &ParsedInput) -> u64 {
    let mut stream = BitStream::new(input, 4);
    let packet = Packet::parse(&mut stream).unwrap();
    packet.version_sum()
}

fn solve_part_two(_input: &ParsedInput) -> u64 {
    let mut stream = BitStream::new(_input, 4);
    let packet = Packet::parse(&mut stream).unwrap();
    packet.execute()
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
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_literal_packet() {
        let input = "D2FE28";
        let input = parse_input(input);
        let mut stream = BitStream::new(&input, 4);
        let packet = Packet::parse(&mut stream).unwrap();

        assert_eq!(packet.version(), 6);
        assert!(matches!(packet, Packet::Literal(p) if p.literal == 2021));
    }

    #[test]
    fn parse_operator_tid_0() {
        let input = "38006F45291200";

        let input = parse_input(input);
        let mut stream = BitStream::new(&input, 4);
        let packet = Packet::parse(&mut stream).unwrap();

        assert_eq!(packet.version(), 1);
        assert!(matches!(packet, Packet::Operator(_)));
    }

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 31);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 949);
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
        assert_eq!(result, 54);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1114600142730);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
