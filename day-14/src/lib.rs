use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    error::Error,
    multi::separated_list1,
    sequence::{pair, terminated, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (r, (molecule, inserts)) = parse_input(input).unwrap();
    assert!(r.is_empty());
    compute(molecule, inserts, 10)
}

pub fn process_part2(input: &str) -> String {
    let (r, (molecule, inserts)) = parse_input(input).unwrap();
    assert!(r.is_empty());
    compute(molecule, inserts, 40)
}

fn compute(molecule: Molecule, inserts: Inserts, iterations: usize) -> String {
    let mut map1: HashMap<(char, char), u128> =
        inserts.keys().map(|&combo| (combo, 0_u128)).collect();
    molecule.windows(2).for_each(|w| {
        let chars = (w[0], w[1]);
        *map1.entry(chars).or_insert(0) += 1;
    });
    let mut map2 = map1.clone();

    (0..iterations).for_each(|_i| {
        map2 = map1.clone();
        map1.iter().for_each(|(&(c1, c2), &v)| {
            if let Some(&insert) = inserts.get(&(c1, c2)) {
                map2.entry((c1, c2)).and_modify(|count| *count -= v);
                *map2.entry((c1, insert)).or_insert(0) += v;
                *map2.entry((insert, c2)).or_insert(0) += v;
            }
        });
        std::mem::swap(&mut map1, &mut map2);
    });
    let mut counts = HashMap::new();
    map1.into_iter().for_each(|((c1, _), v)| {
        *counts.entry(c1).or_insert(0) += v;
    });
    counts
        .entry(*molecule.last().unwrap())
        .and_modify(|v| *v += 1);
    let (_, &max) = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let (_, min) = counts.into_iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    (max - min).to_string()
}

type Molecule = Vec<char>;
type Inserts = HashMap<(char, char), char>;

fn parse_input(input: &str) -> IResult<&str, (Molecule, Inserts)> {
    let (input, molecule) = terminated(alpha1, pair(newline, newline))(input)?;
    let molecule: Vec<_> = molecule.chars().collect();
    let (input, inserts) = separated_list1(newline, parse_insert)(input)?;
    Ok((input, (molecule, inserts.into_iter().collect())))
}

fn parse_insert(input: &str) -> IResult<&str, ((char, char), char)> {
    let (input, (left, right, _, c)) = tuple((anychar, anychar, tag(" -> "), anychar))(input)?;
    if ![left, right, c].iter().all(|ch| ch.is_ascii_uppercase()) {
        Err(nom::Err::Failure(Error {
            input,
            code: nom::error::ErrorKind::Char,
        }))
    } else {
        Ok((input, ((left, right), c)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let result = process_part1(input);
        assert_eq!(result, "1588");
    }

    #[test]
    fn part2() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let result = process_part2(input);
        assert_eq!(result, "2188189693529");
    }
}
