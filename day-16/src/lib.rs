#![allow(warnings)]
use bitvec::prelude::*;
use std::collections::{HashMap, HashSet};

type ProblemInput = BitVec<Msb0, u32>;
fn parse(input: &str) -> ProblemInput {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => bitvec![0, 0, 0, 0],
            '1' => bitvec![0, 0, 0, 1],
            '2' => bitvec![0, 0, 1, 0],
            '3' => bitvec![0, 0, 1, 1],
            '4' => bitvec![0, 1, 0, 0],
            '5' => bitvec![0, 1, 0, 1],
            '6' => bitvec![0, 1, 1, 0],
            '7' => bitvec![0, 1, 1, 1],
            '8' => bitvec![1, 0, 0, 0],
            '9' => bitvec![1, 0, 0, 1],
            'A' => bitvec![1, 0, 1, 0],
            'B' => bitvec![1, 0, 1, 1],
            'C' => bitvec![1, 1, 0, 0],
            'D' => bitvec![1, 1, 0, 1],
            'E' => bitvec![1, 1, 1, 0],
            'F' => bitvec![1, 1, 1, 1],
            x => unreachable!("got bad input character: {}", x),
        })
        .flatten()
        .collect()
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal(Vec<u64>),
    Operator(Operation, LengthTypeId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FullPacket {
    Literal(u64),
    Operator(Operation, Vec<FullPacket>),
}

impl FullPacket {
    fn resolve(self) -> u64 {
        match self {
            FullPacket::Literal(v) => v,
            FullPacket::Operator(op, packs) => evaluate(
                op,
                &packs.into_iter().map(|z| z.resolve()).collect::<Vec<u64>>(),
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum LengthTypeId {
    Zero(u32),
    One(u32),
}
fn decode_packet(i: &BitSlice<Msb0, u32>) -> Option<(u8, Packet, &BitSlice<Msb0, u32>)> {
    let (version, rest) = i.split_at(3);
    let (type_id, mut rest) = rest.split_at(3);

    let mut padding = bitvec![Msb0, u32; 0].repeat(29);
    padding.extend(type_id);
    let type_id_n = padding.as_raw_slice()[0];
    if type_id_n == 4 {
        let mut content: Vec<u64> = vec![];

        loop {
            let (c, r) = rest.split_at(5);
            rest = r;
            let (has_next, v) = c.split_first().unwrap();
            let mut x = v.to_bitvec();
            let mut padding = bitvec![Msb0, u32; 0].repeat(28);
            padding.extend(x);
            content.push(padding.as_raw_slice()[0] as u64);
            if !has_next {
                break;
            }
        }
        let mut padding = bitvec![Msb0, u32; 0].repeat(29);
        padding.extend(version);
        dbg!(&padding);
        return Some((
            padding.as_raw_slice()[0] as u8,
            Packet::Literal(content),
            rest,
        ));
    } else {
        let (t, rest) = rest.split_first()?;
        let (length_type_id, rest) = if *t {
            let (content, rest) = rest.split_at(11);
            let mut padding = bitvec![Msb0, u32; 0].repeat(21);
            padding.extend(content);
            (LengthTypeId::One(padding.as_raw_slice()[0] as u32), rest)
        } else {
            let (content, rest) = rest.split_at(15);
            let mut padding = bitvec![Msb0, u32; 0].repeat(17);
            padding.extend(content);
            (LengthTypeId::Zero(padding.as_raw_slice()[0] as u32), rest)
        };
        let mut padding = bitvec![Msb0, u32; 0].repeat(29);
        use Operation::*;
        let operation = match type_id_n {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            c => unreachable!("Invalid type_id_number: {}", c),
        };
        padding.extend(version);
        // dbg!(&padding);
        return Some((
            padding.as_raw_slice()[0] as u8,
            Packet::Operator(operation, length_type_id),
            rest,
        ));
    }
}

fn decode_packet_rec(i: &BitSlice<Msb0, u32>) -> Option<(u8, FullPacket, &BitSlice<Msb0, u32>)> {
    let (version, rest) = i.split_at(3);
    let (type_id, mut rest) = rest.split_at(3);

    let mut padding = bitvec![Msb0, u32; 0].repeat(29);
    padding.extend(type_id);
    let type_id_n = padding.as_raw_slice()[0];
    if type_id_n == 4 {
        let mut content = bitvec![Msb0, u32;];

        loop {
            let (c, r) = rest.split_at(5);
            rest = r;
            let (has_next, v) = c.split_first().unwrap();
            let mut x = v.to_bitvec();
            content.extend(x);
            if !has_next {
                break;
            }
        }
        let mut padding = bitvec![Msb0, u32; 0].repeat(29);
        padding.extend(version);
        let l = content.len();
        let mut padded_content = bitvec![Msb0, u64;0].repeat(64 - l);
        padded_content.extend(content);
        return Some((
            padding.as_raw_slice()[0] as u8,
            FullPacket::Literal(padded_content.as_raw_slice()[0] as u64),
            rest,
        ));
    } else {
        let (t, rest) = rest.split_first()?;
        let (length_type_id, rest) = if *t {
            let (content, rest) = rest.split_at(11);
            let mut padding = bitvec![Msb0, u32; 0].repeat(21);
            padding.extend(content);
            (LengthTypeId::One(padding.as_raw_slice()[0] as u32), rest)
        } else {
            let (content, rest) = rest.split_at(15);
            let mut padding = bitvec![Msb0, u32; 0].repeat(17);
            padding.extend(content);
            (LengthTypeId::Zero(padding.as_raw_slice()[0] as u32), rest)
        };
        let mut padding = bitvec![Msb0, u32; 0].repeat(29);
        use Operation::*;
        let operation = match type_id_n {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            c => unreachable!("Invalid type_id_number: {}", c),
        };
        padding.extend(version);
        let (rest, inner_data) = match length_type_id {
            LengthTypeId::Zero(z) => {
                let mut rezt = rest;
                let start = rest.len();
                let mut inner_data = vec![];
                while start - rezt.len() < z as usize {
                    let x = decode_packet_rec(rezt).unwrap();
                    inner_data.push(x.1);
                    rezt = x.2;
                }
                (rezt, inner_data)
            }
            LengthTypeId::One(n) => {
                let mut rezt = rest;
                let mut inner_data = vec![];
                while inner_data.len() < n as usize {
                    let x = decode_packet_rec(rezt).unwrap();
                    inner_data.push(x.1);
                    rezt = x.2;
                }
                (rezt, inner_data)
            }
        };
        return Some((
            padding.as_raw_slice()[0] as u8,
            FullPacket::Operator(operation, inner_data),
            rest,
        ));
    }
}

type Part1Output = u64;
fn part_1(input: ProblemInput) -> Part1Output {
    let mut res = input.as_bitslice();
    let mut version_sum = 0;
    while let Some((version, packet, rez)) = decode_packet(res) {
        res = rez;
        dbg!(&res);
        version_sum += version as u64;
        if res.count_ones() == 0 {
            break;
        }
    }
    version_sum
}

type Part2Output = u64;
fn part_2(input: ProblemInput) -> Part2Output {
    let mut res = input.as_bitslice();
    let (version, packet, rez) = decode_packet_rec(res).unwrap();
    packet.resolve()
}

fn evaluate(op: Operation, children: &[u64]) -> u64 {
    use Operation::*;
    match op {
        Sum => children.iter().sum(),
        Product => children.iter().product(),
        Minimum => *children.iter().min().unwrap(),
        Maximum => *children.iter().max().unwrap(),
        GreaterThan => {
            if children[0] > children[1] {
                1
            } else {
                0
            }
        }
        LessThan => {
            if children[0] < children[1] {
                1
            } else {
                0
            }
        }
        EqualTo => {
            if children[0] == children[1] {
                1
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    fn test_part_1(input: &str, expected: Part1Output) {
        assert_eq!(part_1(parse(input)), expected);
    }

    fn test_part_2(input: &str, expected: Part2Output) {
        assert_eq!(part_2(parse(input)), expected);
    }

    const EXAMPLE: &str = r#"
    8A004A801A8002F478
    "#;

    #[test]
    fn test_parse() {
        assert_eq!(parse("12"), bitvec![0, 0, 0, 1, 0, 0, 1, 0]);
    }

    #[test]
    fn test_sanity() {
        let mut x = bitvec![0, 1, 1];
        x.reverse();
        assert_eq!(x.as_raw_slice()[0], 0b11);
    }

    #[test]
    fn test_decode_literal() {
        assert_eq!(
            decode_packet(bitvec![Msb0, u32; 0,1,0, 1,0,0, 1,0,0,0,1, 0,0,0,1,0].as_bitslice())
                .unwrap(),
            (
                2,
                Packet::Literal(vec![1, 2]),
                bitvec![Msb0,u32;].as_bitslice()
            )
        );
    }

    #[test]
    fn test_decode_literal_rec() {
        assert_eq!(
            decode_packet_rec(bitvec![Msb0, u32; 0,1,0, 1,0,0, 1,0,0,0,1, 0,0,0,1,0].as_bitslice())
                .unwrap(),
            (2, FullPacket::Literal(18), bitvec![Msb0,u32;].as_bitslice())
        );
    }

    #[test]
    fn test_decode_operator() {
        assert_eq!(decode_packet(bitvec![Msb0, u32; 0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,1,1,0,1,0,0,0,1,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,]
            .as_bitslice()
        ).unwrap(),
                   (1, Packet::Operator(Operation::LessThan, LengthTypeId::Zero(27)),
                    bitvec![Msb0,u32; 1,1,0,1,0,0,0,1,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0]
                        .as_bitslice()));
    }

    #[test]
    fn test_decode_operator_rec() {
        assert_eq!(decode_packet_rec(bitvec![Msb0, u32; 0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,1,1,0,1,0,0,0,1,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,]
            .as_bitslice()
        ).unwrap(),
                   (1, FullPacket::Operator(Operation::LessThan, vec![FullPacket::Literal(10), FullPacket::Literal(20)]),
                    bitvec![Msb0,u32; 0].repeat(7)
                        .as_bitslice()));
    }

    #[test]
    fn test_resolve_rec() {
        let packet = FullPacket::Operator(
            Operation::LessThan,
            vec![FullPacket::Literal(10), FullPacket::Literal(20)],
        );
        assert_eq!(packet.resolve(), 1);
    }

    #[test]
    fn part_1_on_example() {
        test_part_1(EXAMPLE, 16);
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(&read_to_string("input.txt").unwrap(), 984);
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(EXAMPLE, Part2Output::default());
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(&read_to_string("input.txt").unwrap(), 0);
    }
}
