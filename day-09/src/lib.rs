use std::collections::VecDeque;

use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, input) = parse_input(input).unwrap();
    let mut height_map: Vec<Vec<u8>> = vec![vec![99; input[0].len() + 2]];
    height_map.extend(
        input
            .iter()
            .map(|s| {
                let mut v = Vec::<u8>::new();
                v.push(99);
                v.extend(
                    s.chars()
                        .map(|x| x.to_digit(10).expect("Need a valid digit!") as u8)
                        .collect::<Vec<u8>>(),
                );
                v.push(99);
                v
            })
            .collect::<Vec<Vec<u8>>>(),
    );
    height_map.push(vec![99; input[0].len() + 2]);

    let mut low_points = Vec::new();
    let mut risk_level = 0_usize;
    for y in 1..(height_map.len() - 1) {
        for x in 1..(height_map[0].len() - 1) {
            let height = height_map[y][x];
            let above = height_map[y - 1][x];
            let to_left = height_map[y][x - 1];
            let to_right = height_map[y][x + 1];
            let below = height_map[y + 1][x];
            if height < above && height < to_left && height < to_right && height < below {
                risk_level += (height + 1) as usize;
                low_points.push((x, y));
            }
        }
    }
    risk_level.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, input) = parse_input(input).unwrap();
    let mut height_map: Vec<Vec<u8>> = vec![vec![99; input[0].len() + 2]];
    height_map.extend(
        input
            .iter()
            .map(|s| {
                let mut v = Vec::<u8>::new();
                v.push(99);
                v.extend(
                    s.chars()
                        .map(|x| x.to_digit(10).expect("Need a valid digit!") as u8)
                        .collect::<Vec<u8>>(),
                );
                v.push(99);
                v
            })
            .collect::<Vec<Vec<u8>>>(),
    );
    height_map.push(vec![99; input[0].len() + 2]);

    let mut low_points = Vec::new();
    for y in 1..(height_map.len() - 1) {
        for x in 1..(height_map[0].len() - 1) {
            let height = height_map[y][x];
            let above = height_map[y - 1][x];
            let to_left = height_map[y][x - 1];
            let to_right = height_map[y][x + 1];
            let below = height_map[y + 1][x];
            if height < above && height < to_left && height < to_right && height < below {
                low_points.push((x, y));
            }
        }
    }
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            match height_map[y][x] {
                9 | 99 => height_map[y][x] = 0,
                _ => height_map[y][x] = 1,
            }
        }
    }

    let mut basin_sizes = Vec::new();
    for (start_x, start_y) in low_points.into_iter() {
        if height_map[start_y][start_x] == 0 {
            continue; // low point already part of larger basin
        }
        let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
        deque.push_back((start_x, start_y));
        let mut basin_count = 0;
        while !deque.is_empty() {
            let (x, y) = deque.pop_front().unwrap();
            if height_map[y][x] == 0 {
                continue;
            }
            height_map[y][x] = 0;
            basin_count += 1;
            let coords = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (new_x, new_y) in coords.into_iter() {
                if height_map[new_y][new_x] == 1 {
                    deque.push_back((new_x, new_y));
                }
            }
        }
        basin_sizes.push(basin_count);
    }

    basin_sizes.sort();
    let end = basin_sizes.len() - 1;
    let answer = basin_sizes[end] * basin_sizes[end - 1] * basin_sizes[end - 2];
    answer.to_string()
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
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let result = process_part1(input);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let result = process_part2(input);
        assert_eq!(result, "1134");
    }
}
