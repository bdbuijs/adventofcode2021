use std::collections::{BinaryHeap, HashMap};

use nom::{
    character::complete::digit1, character::complete::newline, multi::separated_list1, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (r, cave) = parse_input(input).unwrap();
    assert!(r.is_empty());
    find_path((0, 0), (cave.len() - 1, cave.len() - 1), &cave)
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (r, mut cave) = parse_input(input).unwrap();
    assert!(r.is_empty());
    cave = expand_cave(cave);
    find_path((0, 0), (cave.len() - 1, cave.len() - 1), &cave)
        .unwrap()
        .to_string()
}

fn find_path(start: (usize, usize), end: (usize, usize), cave: &Vec<Vec<u32>>) -> Option<u32> {
    let mut distances: HashMap<(usize, usize), u32> = (0..cave.len())
        .flat_map(|x| (0..cave[0].len()).map(move |y| ((x, y), u32::MAX)))
        .collect();
    let mut heap = BinaryHeap::new();

    *distances.get_mut(&start).unwrap() = 0;
    heap.push(Node {
        location: start,
        distance: 0,
        path: vec![start],
    });

    while let Some(Node {
        location,
        distance,
        path,
    }) = heap.pop()
    {
        if location == end {
            return Some(distance);
        }
        if distance > distances[&location] {
            continue;
        }

        for (neighbour, weight) in neighbours(location, cave, &path) {
            let next_distance = distance + weight;
            if next_distance < distances[&neighbour] {
                *distances.get_mut(&neighbour).unwrap() = next_distance;
                let mut next_path = path.clone();
                next_path.push(neighbour);
                heap.push(Node {
                    location: neighbour,
                    distance: next_distance,
                    path: next_path,
                });
            }
        }
    }

    None
}

fn neighbours(
    location: (usize, usize),
    grid: &[Vec<u32>],
    path: &[(usize, usize)],
) -> Vec<((usize, usize), u32)> {
    let mut neighbours: Vec<((usize, usize), u32)> = Vec::new();
    let (x, y) = location;
    let rows = grid.len() - 1;
    let cols = grid[0].len() - 1;

    if x > 0 && !path.contains(&(x - 1, y)) {
        neighbours.push(((x - 1, y), grid[y][x - 1]));
    }
    if y > 0 && !path.contains(&(x, y - 1)) {
        neighbours.push(((x, y - 1), grid[y - 1][x]));
    }
    if x < rows && !path.contains(&(x + 1, y)) {
        neighbours.push(((x + 1, y), grid[y][x + 1]));
    }
    if y < cols && !path.contains(&(x, y + 1)) {
        neighbours.push(((x, y + 1), grid[y + 1][x]));
    }

    neighbours
}

fn expand_cave(cave: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut cave = cave;
    cave = cave
        .into_iter()
        .map(|row| {
            let mut new_row = Vec::new();
            (0_u32..5).for_each(|offset| {
                row.iter().for_each(|&risk| {
                    let mut v = risk + offset;
                    if v > 9 {
                        v -= 9;
                    }
                    new_row.push(v);
                })
            });
            new_row
        })
        .collect();
    cave = (0..5)
        .flat_map(|offset| {
            cave.iter().map(move |row| {
                row.iter()
                    .map(|&risk| {
                        let mut v = risk + offset;
                        if v > 9 {
                            v -= 9;
                        }
                        v
                    })
                    .collect()
            })
        })
        .collect();
    cave
}

struct Node {
    location: (usize, usize),
    distance: u32,
    path: Vec<(usize, usize)>,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl std::cmp::Eq for Node {}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type Line = Vec<u32>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = digit1(input)?;
    Ok((
        input,
        line.chars().map(|c| c.to_digit(10).unwrap()).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let result = process_part1(input);
        assert_eq!(result, "40");
    }

    #[test]
    fn part2() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let result = process_part2(input);
        assert_eq!(result, "315");
    }
}
