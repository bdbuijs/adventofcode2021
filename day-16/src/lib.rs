use nom::{
    character::complete::char as nomchar,
    character::complete::one_of,
    combinator::recognize,
    multi::{many1, many_m_n},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, bits) = parse_input(input).unwrap();
    let (_, packet) = parse_input_bits(&bits).unwrap();
    packet.version_sum().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, bits) = parse_input(input).unwrap();
    let (_, packet) = parse_input_bits(&bits).unwrap();
    packet.expression().to_string()
}

fn parse_input(input: &str) -> IResult<&str, String> {
    let (input, hex_chars) = many1(one_of("0123456789ABCDEF"))(input)?;
    let mut bits = String::with_capacity(hex_chars.len() * 4);
    hex_chars.into_iter().for_each(|c| {
        bits.push_str(match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!(),
        })
    });

    Ok((input, bits))
}

fn parse_input_bits(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = parse_packet(input)?;

    Ok((input, packet))
}

fn parse_bits(input: &str, count: usize) -> IResult<&str, &str> {
    let (input, bits) = recognize(many_m_n(count, count, recognize(one_of("01"))))(input)?;
    Ok((input, bits))
}

fn parse_binary_number(input: &str) -> IResult<&str, u32> {
    let n = input
        .chars()
        .map(|c| c.to_digit(2).unwrap())
        .fold(0, |acc, d| (acc << 1) + d);
    Ok((input, n))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = parse_version(input)?;
    let (input, id) = parse_id(input)?;
    // dbg!(id);
    match id {
        4 => {
            // literal
            // dbg!("literal", input);
            let (input, value) = parse_literal(input)?;
            // dbg!(input, value);
            Ok((input, Packet::Literal(version, value)))
        }
        _ => {
            // operator
            let (input, length_type) = one_of("01")(input)?;
            // dbg!(length_type);
            match length_type {
                '0' => {
                    let (input, bit_length) = parse_bits(input, 15)?;
                    let (_, bit_length) = parse_binary_number(bit_length)?;
                    // dbg!(bit_length);
                    let (input, packet_bits) = parse_bits(input, bit_length as usize)?;
                    // dbg!(packet_bits);
                    let (_, packets) = many1(parse_packet)(packet_bits)?;
                    let packet = Packet::Operator(version, id, packets);
                    Ok((input, packet))
                }
                '1' => {
                    let (input, sub_packet_count) = parse_bits(input, 11)?;
                    let (_, sub_packet_count) = parse_binary_number(sub_packet_count)?;
                    let sub_packet_count = sub_packet_count as usize;
                    // dbg!(sub_packet_count);
                    let (input, packets) =
                        many_m_n(sub_packet_count, sub_packet_count, parse_packet)(input)?;
                    let packet = Packet::Operator(version, id, packets);
                    Ok((input, packet))
                }
                _ => unreachable!(),
            }
        }
    }
}

fn parse_version(input: &str) -> IResult<&str, u32> {
    let (input, vstr) = parse_bits(input, 3)?;
    let (_, n) = parse_binary_number(vstr)?;
    Ok((input, n))
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    let (input, istr) = parse_bits(input, 3)?;
    let (_, n) = parse_binary_number(istr)?;
    Ok((input, n))
}

fn parse_literal(input: &str) -> IResult<&str, u128> {
    let mut output = 0_u128;
    let mut input = input;
    let mut bstr;
    while let Ok((remaining_input, _)) = nomchar::<&str, nom::error::Error<_>>('1')(input) {
        (input, bstr) = parse_bits(remaining_input, 4)?;
        let (_, next_bits) = parse_binary_number(bstr)?;
        let next_bits = next_bits as u128;
        output <<= 4;
        output += next_bits;
    }
    let (input, _) = nomchar('0')(input)?;
    let (input, bstr) = parse_bits(input, 4)?;
    let (_, end_bits) = parse_binary_number(bstr)?;
    let end_bits = end_bits as u128;
    output <<= 4;
    output += end_bits;
    Ok((input, output))
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    // version, value
    Literal(u32, u128),
    // version, id, sub-packets
    Operator(u32, u32, Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Self::Literal(v, _) => *v,
            Self::Operator(v, _, subs) => *v + subs.iter().map(|p| p.version_sum()).sum::<u32>(),
        }
    }

    fn expression(&self) -> u128 {
        match self {
            Self::Literal(_, value) => *value,
            Self::Operator(_, id, packets) => {
                match *id {
                    // Packets with type ID 0 are sum packets
                    0 => packets.iter().map(|p| p.expression()).sum(),
                    // Packets with type ID 1 are product packets
                    1 => packets.iter().map(|p| p.expression()).product(),
                    // Packets with type ID 2 are minimum packets
                    2 => packets.iter().map(|p| p.expression()).min().unwrap(),
                    // Packets with type ID 3 are maximum packets
                    3 => packets.iter().map(|p| p.expression()).max().unwrap(),
                    // Packets with type ID 5 are greater than packets
                    5 => {
                        if packets[0].expression() > packets[1].expression() {
                            1
                        } else {
                            0
                        }
                    }
                    // Packets with type ID 6 are less than packets
                    6 => {
                        if packets[0].expression() < packets[1].expression() {
                            1
                        } else {
                            0
                        }
                    }
                    // Packets with type ID 7 are equal to packets
                    7 => {
                        if packets[0].expression() == packets[1].expression() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let input = "D2FE28";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        let value = match packet {
            Packet::Literal(_, value) => value,
            Packet::Operator(_, _, _) => panic!("Not a valid literal!"),
        };
        assert_eq!(value, 2021);
    }

    #[test]
    fn operator1() {
        let input = "38006F45291200";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        let expected_packet =
            Packet::Operator(1, 6, vec![Packet::Literal(6, 10), Packet::Literal(2, 20)]);
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn operator2() {
        let input = "EE00D40C823060";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        let expected_packet = Packet::Operator(
            7,
            3,
            vec![
                Packet::Literal(2, 1),
                Packet::Literal(4, 2),
                Packet::Literal(1, 3),
            ],
        );
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn version_sum1() {
        let input = "8A004A801A8002F478";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn version_sum2() {
        let input = "620080001611562C8802118E34";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn version_sum3() {
        let input = "C0015000016115A2E0802F182340";
        let (_, bits) = parse_input(input).unwrap();
        dbg!(&bits);
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.version_sum(), 23);
    }

    #[test]
    fn version_sum4() {
        let input = "A0016C880162017C3686B18A3D4780";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn expression1() {
        let input = "C200B40A82";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 3);
    }

    #[test]
    fn expression2() {
        let input = "04005AC33890";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 54);
    }

    #[test]
    fn expression3() {
        let input = "880086C3E88112";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 7);
    }

    #[test]
    fn expression4() {
        let input = "CE00C43D881120";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 9);
    }

    #[test]
    fn expression5() {
        let input = "D8005AC2A8F0";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 1);
    }

    #[test]
    fn expression6() {
        let input = "F600BC2D8F";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 0);
    }

    #[test]
    fn expression7() {
        let input = "9C005AC2F8F0";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 0);
    }

    #[test]
    fn expression8() {
        let input = "9C0141080250320F1802104A08";
        let (_, bits) = parse_input(input).unwrap();
        let (_, packet) = parse_input_bits(&bits).unwrap();
        dbg!(&packet);
        assert_eq!(packet.expression(), 1);
    }
}
