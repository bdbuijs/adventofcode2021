use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as nomchar,
    character::complete::{digit1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut depth = 0;
    let mut position = 0;
    instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            Instruction::Forward(x) => position += x,
            Instruction::Up(x) => depth -= x,
            Instruction::Down(x) => depth += x,
        });
    (depth * position).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            Instruction::Forward(x) => {
                position += x;
                depth += x * aim;
            }
            Instruction::Up(x) => aim -= x,
            Instruction::Down(x) => aim += x,
        });
    (depth * position).to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

type Line = Instruction;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (dir, _, amount)) = tuple((parse_direction, nomchar(' '), digit1))(input)?;
    let direction = match dir {
        "forward" => Instruction::Forward(amount.parse().unwrap()),
        "down" => Instruction::Down(amount.parse().unwrap()),
        "up" => Instruction::Up(amount.parse().unwrap()),
        _ => unreachable!(),
    };
    Ok((input, direction))
}

fn parse_direction(input: &str) -> IResult<&str, &str> {
    alt((tag("forward"), tag("down"), tag("up")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let result = process_part1(input);
        assert_eq!(result, "150");
    }

    #[test]
    fn part2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let result = process_part2(input);
        assert_eq!(result, "900");
    }
}
