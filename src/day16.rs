use crate::day16::Operator::{Maximum, Minimum, Product, Sum, Unknown, EQ, GT, LT};
use itertools::Itertools;

fn parse_hexa(input: &str) -> Vec<char> {
    input
        .chars()
        .flat_map(|c| match c {
            '0' => "0000".chars(),
            '1' => "0001".chars(),
            '2' => "0010".chars(),
            '3' => "0011".chars(),
            '4' => "0100".chars(),
            '5' => "0101".chars(),
            '6' => "0110".chars(),
            '7' => "0111".chars(),
            '8' => "1000".chars(),
            '9' => "1001".chars(),
            'A' => "1010".chars(),
            'B' => "1011".chars(),
            'C' => "1100".chars(),
            'D' => "1101".chars(),
            'E' => "1110".chars(),
            'F' => "1111".chars(),
            _ => "".chars(),
        })
        .collect()
}
fn parse_bit(c: char) -> Option<usize> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        o => {
            eprintln!("cannot parse '{}", o);
            None
        }
    }
}
fn bits_to_num(bits: &mut dyn Iterator<Item = char>, bit_count: usize) -> usize {
    bits.take(bit_count)
        .enumerate()
        .filter_map(|(i, c)| parse_bit(c).map(|v| v << (bit_count - 1 - i)))
        .sum()
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GT,
    LT,
    EQ,
    Unknown(usize),
}

impl Operator {
    fn new(id: usize) -> Self {
        match id {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GT,
            6 => LT,
            7 => EQ,
            _ => Unknown(id),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Operation {
    op: Operator,
    operands: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
enum PacketContent {
    Literal(usize),
    Operation(Operation),
}
impl PacketContent {
    fn parse_literal(bits: &mut dyn Iterator<Item = char>) -> usize {
        let mut value: usize = 0;
        loop {
            let last_quartet = bits.next().unwrap() == '0';
            let val = bits_to_num(bits, 4);
            value = (value << 4) + val;
            if last_quartet {
                break;
            }
        }
        value
    }
    fn parse(bits: &mut dyn Iterator<Item = char>) -> Self {
        let typeid = bits_to_num(bits, 3);
        match typeid {
            4 => PacketContent::Literal(PacketContent::parse_literal(bits)),
            id => PacketContent::Operation(PacketContent::parse_operation(id, bits)),
        }
    }
    fn parse_operation(typeid: usize, bits: &mut dyn Iterator<Item = char>) -> Operation {
        let op = Operator::new(typeid);
        let length_type = bits.next().unwrap();
        match length_type {
            '0' => {
                let subpackets_length = bits_to_num(bits, 15);
                // TODO : see if bits are consumed as well
                let mut subpackets_bits = bits.take(subpackets_length).peekable();
                let mut operands = vec![];
                while subpackets_bits.peek().is_some() {
                    operands.push(Packet::parse(&mut subpackets_bits));
                }
                Operation { op, operands }
            }
            _ => {
                let operands_count = bits_to_num(bits, 11);
                let operands = (0..operands_count).map(|_| Packet::parse(bits)).collect();
                Operation { op, operands }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    content: PacketContent,
}

impl Packet {
    fn from_str(input: &str) -> Self {
        Self::parse(&mut parse_hexa(input).into_iter())
    }
    fn parse(bits: &mut dyn Iterator<Item = char>) -> Self {
        let version = bits_to_num(bits, 3);
        let content = PacketContent::parse(bits);
        Self { version, content }
    }
    fn sum_version(&self) -> usize {
        match &self.content {
            PacketContent::Literal(_) => self.version,
            PacketContent::Operation(op) => {
                op.operands
                    .iter()
                    .map(|operand| operand.sum_version())
                    .sum::<usize>()
                    + self.version
            }
        }
    }

    fn value(&self) -> usize {
        match &self.content {
            PacketContent::Literal(v) => *v,
            PacketContent::Operation(op) => match op.op {
                Sum => op.operands.iter().map(|p| p.value()).sum(),
                Product => op.operands.iter().map(|p| p.value()).product(),
                Minimum => op.operands.iter().map(|p| p.value()).min().unwrap(),
                Maximum => op.operands.iter().map(|p| p.value()).max().unwrap(),
                GT => {
                    let (first, second) = op
                        .operands
                        .iter()
                        .map(|p| p.value())
                        .collect_tuple()
                        .unwrap();
                    if first > second {
                        1
                    } else {
                        0
                    }
                }
                LT => {
                    let (first, second) = op
                        .operands
                        .iter()
                        .map(|p| p.value())
                        .collect_tuple()
                        .unwrap();
                    if first < second {
                        1
                    } else {
                        0
                    }
                }
                EQ => {
                    let (first, second) = op
                        .operands
                        .iter()
                        .map(|p| p.value())
                        .collect_tuple()
                        .unwrap();
                    if first == second {
                        1
                    } else {
                        0
                    }
                }
                Unknown(i) => {
                    panic!("unknown operator {}", i);
                }
            },
        }
    }
}

pub fn print_bits() {
    let input = include_str!("../resources/day16_bits.txt");
    println!(
        "summed BITS versions : {}",
        Packet::from_str(input).sum_version()
    );
    println!("BITS value : {}", Packet::from_str(input).value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples_work() {
        let input = "D2FE28";
        let mut bits = parse_hexa(input).into_iter();
        assert_eq!(
            Packet {
                version: 6,
                content: PacketContent::Literal(2021)
            },
            Packet::parse(&mut bits)
        );

        let op = Packet::parse(&mut parse_hexa("38006F45291200").into_iter());
        assert_eq!(
            Packet {
                version: 1,
                content: PacketContent::Operation(Operation {
                    op: LT,
                    operands: vec![
                        Packet {
                            version: 6,
                            content: PacketContent::Literal(10)
                        },
                        Packet {
                            version: 2,
                            content: PacketContent::Literal(20)
                        },
                    ]
                }),
            },
            op
        );

        let op = Packet::parse(&mut parse_hexa("EE00D40C823060").into_iter());
        assert_eq!(
            Packet {
                version: 7,
                content: PacketContent::Operation(Operation {
                    op: Maximum,
                    operands: vec![
                        Packet {
                            version: 2,
                            content: PacketContent::Literal(1)
                        },
                        Packet {
                            version: 4,
                            content: PacketContent::Literal(2)
                        },
                        Packet {
                            version: 1,
                            content: PacketContent::Literal(3)
                        },
                    ]
                }),
            },
            op
        );

        assert_eq!(16, Packet::from_str("8A004A801A8002F478").sum_version());
        assert_eq!(
            12,
            Packet::from_str("620080001611562C8802118E34").sum_version()
        );
        assert_eq!(
            23,
            Packet::from_str("C0015000016115A2E0802F182340").sum_version()
        );
        assert_eq!(
            31,
            Packet::from_str("A0016C880162017C3686B18A3D4780").sum_version()
        );

        assert_eq!(3, Packet::from_str("C200B40A82").value());
        assert_eq!(54, Packet::from_str("04005AC33890").value());
        assert_eq!(7, Packet::from_str("880086C3E88112").value());
        assert_eq!(9, Packet::from_str("CE00C43D881120").value());
        assert_eq!(1, Packet::from_str("D8005AC2A8F0").value());
        assert_eq!(0, Packet::from_str("F600BC2D8F").value());
        assert_eq!(0, Packet::from_str("9C005AC2F8F0").value());
        assert_eq!(1, Packet::from_str("9C0141080250320F1802104A08").value());
    }
}
