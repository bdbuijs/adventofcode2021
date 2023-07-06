use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::i64 as nomi64, sequence::terminated, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, target_area) = parse_input(input).unwrap();
    let launch_point = Point { x: 0, y: 0 };
    let mut highest = 0;
    for initial_x_vel in 1..=target_area.right {
        for initial_y_vel in target_area.bottom..100 {
            let probe = Probe::new(launch_point, initial_x_vel, initial_y_vel);
            if probe.launch(&target_area) {
                let high = (initial_y_vel * (initial_y_vel + 1)) / 2;
                if high > highest {
                    highest = high;
                }
            }
        }
    }
    highest.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, target_area) = parse_input(input).unwrap();
    let launch_point = Point { x: 0, y: 0 };
    let mut successes = Vec::new();
    for initial_x_vel in 1..=target_area.right {
        for initial_y_vel in target_area.bottom..200 {
            let probe = Probe::new(launch_point, initial_x_vel, initial_y_vel);
            if probe.launch(&target_area) {
                successes.push((initial_x_vel, initial_y_vel));
            }
        }
    }
    successes.len().to_string()
}

fn parse_input(input: &str) -> IResult<&str, Area> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, startx) = terminated(nomi64, tag(".."))(input)?;
    let (input, endx) = terminated(nomi64, tag(", y="))(input)?;
    let (input, starty) = terminated(nomi64, tag(".."))(input)?;
    let (input, endy) = nomi64(input)?;
    let area = Area::new(startx..=endx, starty..=endy);

    Ok((input, area))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Area {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    bottom: i64,
    left: i64,
    right: i64,
}

impl Area {
    fn new(x_range: RangeInclusive<i64>, y_range: RangeInclusive<i64>) -> Self {
        Self {
            bottom: y_range.clone().min().unwrap(),
            left: x_range.clone().min().unwrap(),
            right: x_range.clone().max().unwrap(),
            x_range,
            y_range,
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.x_range.contains(&point.x) && self.y_range.contains(&point.y)
    }
}

#[derive(Clone)]
struct Probe {
    x: i64,
    y: i64,
    x_vel: i64,
    y_vel: i64,
}

impl Probe {
    fn new(launch_point: Point, x_vel: i64, y_vel: i64) -> Self {
        let Point { x, y } = launch_point;
        Self { x, y, x_vel, y_vel }
    }

    fn step(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        self.x_vel -= self.x_vel.signum();
        self.y_vel -= 1;
    }

    fn chance(&self, area: &Area) -> bool {
        !(self.y < area.bottom && self.y_vel.is_negative()
            || self.x < area.left && !self.x_vel.is_positive()
            || self.x > area.right && !self.x_vel.is_negative())
    }

    fn location(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn launch(mut self, target: &Area) -> bool {
        loop {
            if !self.chance(target) {
                break false;
            }
            self.step();
            if target.contains(self.location()) {
                break true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "target area: x=20..30, y=-10..-5";
        let result = process_part1(input);
        assert_eq!(result, "45");
    }

    #[test]
    fn part2() {
        let input = "target area: x=20..30, y=-10..-5";
        let result = process_part2(input);
        assert_eq!(result, "112");
    }
}
