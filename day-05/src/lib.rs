use std::cmp::Ordering;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    character::complete::{char as nomchar, digit1},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let mut field = HashMap::new();
    lines
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .flat_map(|line| line.iter())
        .for_each(|point| {
            *field.entry(point).or_insert(0_usize) += 1;
        });
    let crossings = field.into_values().filter(|&x| x > 1).count();
    crossings.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let mut field = HashMap::new();
    lines
        .into_iter()
        .flat_map(|line| line.iter())
        .for_each(|point| {
            *field.entry(point).or_insert(0_usize) += 1;
        });
    let crossings = field.into_values().filter(|&x| x > 1).count();
    crossings.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter::new(self.clone())
    }
}

struct LineIter {
    line: Option<Line>,
    x: i32,
    y: i32,
    x_dif: i32,
    y_dif: i32,
}

impl LineIter {
    fn new(line: Line) -> Self {
        let x_dif = match line.start.x.cmp(&line.end.x) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        let y_dif = match line.start.y.cmp(&line.end.y) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };

        Self {
            x: line.start.x as i32,
            y: line.start.y as i32,
            line: Some(line),
            x_dif,
            y_dif,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = &self.line {
            let (x, y) = (self.x as usize, self.y as usize);
            if x == line.end.x && y == line.end.y {
                Some(self.line.take().unwrap().end)
            } else {
                let point = Point { x, y };
                self.x += self.x_dif;
                self.y += self.y_dif;
                Some(point)
            }
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, start) = parse_point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, end) = parse_point(input)?;
    Ok((input, Line { start, end }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = parse_usize(input)?;
    let (input, _) = nomchar(',')(input)?;
    let (input, y) = parse_usize(input)?;
    Ok((input, Point { x, y }))
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
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = process_part1(input);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = process_part2(input);
        assert_eq!(result, "12");
    }
}
