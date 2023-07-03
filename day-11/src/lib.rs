use std::collections::VecDeque;

use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, input_strs) = parse_input(input).unwrap();
    let mut octopuses = input_strs
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).expect("Need a valid digit!") as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let mut flashes = 0;
    let mut hundred_steps_flashes = 0;
    for step in 1..300 {
        let mut deque = VecDeque::new();

        // energize
        (0..10).for_each(|y| {
            (0..10).for_each(|x| {
                octopuses[y][x] += 1;
                if octopuses[y][x] > 9 {
                    deque.push_back((x, y));
                }
            });
        });

        // flash
        let mut flashed = 0;
        while !deque.is_empty() {
            let (x, y) = deque.pop_front().unwrap();
            octopuses[y][x] = -10;
            flashed += 1;
            flashes += 1;
            for new_y in range(&y) {
                for new_x in range(&x) {
                    octopuses[new_y][new_x] += 1;
                    if octopuses[new_y][new_x] > 9 && !deque.contains(&(new_x, new_y)) {
                        deque.push_back((new_x, new_y));
                    }
                }
            }
        }

        // reset
        (0..10).for_each(|y| {
            (0..10).for_each(|x| {
                if octopuses[y][x] < 0 {
                    octopuses[y][x] = 0;
                }
            });
        });
        if flashed == 100 {
            break;
        }
        if step == 100 {
            hundred_steps_flashes = flashes;
        }
    }
    hundred_steps_flashes.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, input_strs) = parse_input(input).unwrap();
    let mut octopuses = input_strs
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).expect("Need a valid digit!") as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let mut all_together = 0;
    for step in 1..300 {
        let mut deque = VecDeque::new();

        // energize
        (0..10).for_each(|y| {
            (0..10).for_each(|x| {
                octopuses[y][x] += 1;
                if octopuses[y][x] > 9 {
                    deque.push_back((x, y));
                }
            });
        });

        // flash
        let mut flashed = 0;
        while !deque.is_empty() {
            let (x, y) = deque.pop_front().unwrap();
            octopuses[y][x] = -10;
            flashed += 1;
            for new_y in range(&y) {
                for new_x in range(&x) {
                    octopuses[new_y][new_x] += 1;
                    if octopuses[new_y][new_x] > 9 && !deque.contains(&(new_x, new_y)) {
                        deque.push_back((new_x, new_y));
                    }
                }
            }
        }

        // reset
        (0..10).for_each(|y| {
            (0..10).for_each(|x| {
                if octopuses[y][x] < 0 {
                    octopuses[y][x] = 0;
                }
            });
        });
        if flashed == 100 {
            all_together = step;
            break;
        }
    }
    all_together.to_string()
}

fn range(i: &usize) -> core::ops::Range<usize> {
    match i {
        0 => 0..2,
        9 => 8..10,
        _ => (i - 1)..(i + 2),
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    let (input, line) = digit1(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let result = process_part1(input);
        assert_eq!(result, "1656");
    }

    #[test]
    fn part2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let result = process_part2(input);
        assert_eq!(result, "195");
    }
}
