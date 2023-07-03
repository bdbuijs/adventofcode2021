use nom::{
    character::complete::digit1, character::complete::newline, multi::separated_list1, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, depths) = parse_input(input).unwrap();
    let increased: usize = depths
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum();
    increased.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, depths) = parse_input(input).unwrap();
    let mut last_sum = depths[0] + depths[1] + depths[2];
    let increased_window: usize = depths
        .windows(3)
        .map(|window| {
            let new_sum: usize = window.iter().sum();
            if new_sum > last_sum {
                last_sum = new_sum;
                1
            } else {
                last_sum = new_sum;
                0
            }
        })
        .sum();
    increased_window.to_string()
}

type Line = usize;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = digit1(input)?;
    Ok((input, line.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let result = process_part1(input);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let result = process_part2(input);
        assert_eq!(result, "5");
    }
}
