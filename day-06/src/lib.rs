use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of, space0, space1},
    character::complete::{char as nomchar, digit1},
    combinator::recognize,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, mut fishes) = parse_input(input).unwrap();
    for _ in 0..80 {
        let mut new_fishes = Vec::new();
        for fish in fishes.iter_mut() {
            if fish.timer == 0 {
                new_fishes.push(Fish::new());
            }
            fish.age();
        }
        fishes.extend(new_fishes.into_iter());
    }

    fishes.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, fishes) = parse_input(input).unwrap();
    let mut total_fish = fishes.len();
    let mut spawn_on_day = [0_usize; 7];
    for start_fish in fishes.iter() {
        spawn_on_day[start_fish.timer] += 1;
    }
    let mut spawn_on_day8 = 0;
    let mut spawn_on_day7 = 0;
    for day in 0..256 {
        let weekday = day % 7;
        let spawn_now = spawn_on_day[weekday];
        total_fish += spawn_now;
        spawn_on_day[weekday] += spawn_on_day7;
        (spawn_on_day7, spawn_on_day8) = (spawn_on_day8, spawn_now);
    }
    total_fish.to_string()
}

#[derive(Debug)]
struct Fish {
    timer: usize,
}

impl Fish {
    fn new() -> Self {
        Fish { timer: 8 }
    }

    fn age(&mut self) {
        if self.timer == 0 {
            self.timer = 6;
        } else {
            self.timer -= 1;
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Fish>> {
    let (input, lines) = separated_list1(nomchar(','), parse_fish)(input)?;
    Ok((input, lines))
}

fn parse_fish(input: &str) -> IResult<&str, Fish> {
    let (input, timer) = parse_usize(input)?;
    Ok((input, Fish { timer }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, digits) = digit1(input)?;
    Ok((input, digits.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "3,4,3,1,2";
        let result = process_part1(input);
        assert_eq!(result, "5934");
    }

    #[test]
    fn part2() {
        let input = "3,4,3,1,2";
        let result = process_part2(input);
        assert_eq!(result, "26984457539");
    }
}
