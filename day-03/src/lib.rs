use nom::{
    character::complete::digit1, character::complete::newline, multi::separated_list1, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let bit_size: usize = numbers.first().unwrap().len();
    let mut bits = vec![0; bit_size];
    let halflen = numbers.len() / 2;
    for row in numbers.into_iter() {
        for (i, c) in row.chars().enumerate() {
            if c == '1' {
                bits[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for b in bits.into_iter() {
        gamma <<= 1;
        epsilon <<= 1;
        if b > halflen {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    (gamma * epsilon).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let bit_size: usize = numbers.first().unwrap().len();
    // let mut bits = vec![0; bit_size];
    let mut oxygen = numbers.clone();
    for i in 0..bit_size {
        let mut oxygen_bits = vec![0; bit_size];
        for row in oxygen.iter() {
            for (j, c) in row.chars().enumerate() {
                if c == '1' {
                    oxygen_bits[j] += 1;
                }
            }
        }
        let len = oxygen.len() as f64 / 2.0;
        if oxygen_bits[i] as f64 > len {
            oxygen.retain(|x| &x[i..=i] == "1");
        } else if oxygen_bits[i] as f64 == len {
            oxygen.retain(|x| &x[i..=i] == "1");
        } else {
            oxygen.retain(|x| &x[i..=i] == "0");
        }
    }

    let mut scrubber = numbers;
    for i in 0..bit_size {
        let mut scrubber_bits = vec![0; bit_size];
        for row in scrubber.iter() {
            for (j, c) in row.chars().enumerate() {
                if c == '1' {
                    scrubber_bits[j] += 1;
                }
            }
        }
        let len = scrubber.len() as f64 / 2.0;
        if scrubber_bits[i] as f64 > len {
            scrubber.retain(|x| &x[i..=i] == "0");
        } else if scrubber_bits[i] as f64 == len {
            scrubber.retain(|x| &x[i..=i] == "0")
        } else {
            scrubber.retain(|x| &x[i..=i] == "1");
        }
        if scrubber.len() == 1 {
            break;
        }
    }
    let answer = bin_to_usize(oxygen[0]) * bin_to_usize(scrubber[0]);
    answer.to_string()
}

fn bin_to_usize(b: &str) -> usize {
    let mut result = 0;
    for b in b.chars() {
        result <<= 1;
        if b == '1' {
            result += 1;
        }
    }
    result
}

type Line<'a> = &'a str;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = digit1(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let result = process_part1(input);
        assert_eq!(result, "198");
    }

    #[test]
    fn part2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let result = process_part2(input);
        assert_eq!(result, "230");
    }
}
