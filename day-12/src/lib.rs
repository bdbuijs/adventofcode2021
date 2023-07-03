use std::collections::HashMap;

use itertools::Itertools;

use nom::{
    character::complete::char as nomchar,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, graph) = parse_input(input).unwrap();
    let count = graph.count_paths("start", "end");
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut graph) = parse_input(input).unwrap();
    graph.cleanse();
    let count = graph.count_paths2("start", "end");
    count.to_string()
}

struct Graph<'a> {
    adjacency_list: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: &'a str, v: &'a str) {
        self.adjacency_list
            .entry(u)
            .or_insert_with(Vec::new)
            .push(v);
        self.adjacency_list
            .entry(v)
            .or_insert_with(Vec::new)
            .push(u);
    }

    fn count_paths(&self, start: &'a str, end: &'a str) -> usize {
        assert!(self.adjacency_list.contains_key(start));
        assert!(self.adjacency_list.contains_key(end));

        let mut paths = Vec::new();
        let mut stack = Vec::new();
        stack.push((start, vec![start]));

        while let Some((current, path)) = stack.pop() {
            if current == end {
                paths.push(path);
                continue;
            }

            if let Some(neighbours) = self.adjacency_list.get(current) {
                for neighbour in neighbours {
                    if (!path.contains(neighbour)) || neighbour.chars().all(|c| c.is_uppercase()) {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        stack.push((neighbour, new_path));
                    }
                }
            }
        }
        paths.len()
    }

    fn count_paths2(&self, start: &'a str, end: &'a str) -> usize {
        assert!(self.adjacency_list.contains_key(start));
        assert!(self.adjacency_list.contains_key(end));

        let mut paths = Vec::new();
        let mut stack = Vec::new();
        stack.push((start, vec![start]));

        while let Some((current, path)) = stack.pop() {
            if current == end {
                paths.push(path);
                continue;
            }

            if let Some(neighbours) = self.adjacency_list.get(current) {
                for neighbour in neighbours {
                    if neighbour.chars().all(|c| c.is_uppercase())
                        || ((!path.contains(neighbour))
                            || path
                                .iter()
                                .duplicates()
                                .all(|d| d.chars().any(|c| c.is_uppercase())))
                    {
                        let mut new_path = path.clone();
                        new_path.push(neighbour);
                        stack.push((neighbour, new_path));
                    }
                }
            }
        }
        paths.len()
    }

    fn cleanse(&mut self) {
        self.adjacency_list
            .values_mut()
            .for_each(|v| v.retain(|&s| s != "start"))
    }
}

type Line<'a> = (&'a str, &'a str);

fn parse_input(input: &str) -> IResult<&str, Graph> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    let mut graph = Graph::new();
    lines.into_iter().for_each(|(u, v)| {
        graph.add_edge(u, v);
    });
    Ok((input, graph))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (start, _, end)) = tuple((alpha1, nomchar('-'), alpha1))(input)?;
    Ok((input, (start, end)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let result = process_part1(input);
        assert_eq!(result, "10");
    }

    #[test]
    fn part1_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let result = process_part1(input);
        assert_eq!(result, "19");
    }

    #[test]
    fn part1_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let result = process_part1(input);
        assert_eq!(result, "226");
    }

    #[test]
    fn part2_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let result = process_part2(input);
        assert_eq!(result, "36");
    }

    #[test]
    fn part2_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let result = process_part2(input);
        assert_eq!(result, "103");
    }

    #[test]
    fn part2_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let result = process_part2(input);
        assert_eq!(result, "3509");
    }
}
