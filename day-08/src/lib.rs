use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::char as nomchar,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

const DISPLAY_NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

pub fn process_part1(input: &str) -> String {
    let (_, displays) = parse_input(input).unwrap();
    let count: usize = displays
        .iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|y| y.len() < 5 || y.len() == 7)
                .count()
        })
        .sum();
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut displays) = parse_input(input).unwrap();
    let output_sum: usize = displays.iter_mut().map(|x| x.output_value()).sum();
    output_sum.to_string()
}

#[derive(Debug)]
struct Display {
    patterns: Vec<String>,
    output: Vec<String>,
    // segment_mapping: Option<HashMap<char, char>>,
    _output_value: Option<usize>,
}

impl Display {
    fn output_value(&mut self) -> usize {
        if self._output_value.is_none() {
            self.deduce();
        }

        self._output_value.unwrap()
    }

    fn deduce(&mut self) {
        let mut mapping = HashMap::<char, char>::new();
        let mut numbers: Vec<HashSet<char>> = Vec::new();
        for _ in 0..10 {
            numbers.push(HashSet::<char>::new());
        }

        let mut fives: Vec<HashSet<char>> = Vec::new();
        let mut sixes: Vec<HashSet<char>> = Vec::new();

        // find 1, 4, 7 and 8
        for pattern in self.patterns.iter() {
            match pattern.len() {
                2 => numbers[1] = HashSet::from_iter(pattern.chars()),
                3 => numbers[7] = HashSet::from_iter(pattern.chars()),
                4 => numbers[4] = HashSet::from_iter(pattern.chars()),
                5 => fives.push(HashSet::from_iter(pattern.chars())),
                6 => sixes.push(HashSet::from_iter(pattern.chars())),
                7 => numbers[8] = HashSet::from_iter(pattern.chars()),
                _ => continue,
            }
        }

        // segment a is difference between 7 and 1
        mapping.insert(
            *numbers[7]
                .difference(&numbers[1])
                .next()
                .expect("Should be the segment a!"),
            'a',
        );

        // find 6, 9 and 0 to find c, d and e
        for six in sixes.into_iter() {
            if !six.is_superset(&numbers[7]) {
                mapping.insert(
                    *numbers[8]
                        .difference(&six)
                        .next()
                        .expect("Should be the segment c!"),
                    'c',
                );
                numbers[6] = six;
            } else if six.is_superset(&numbers[4]) {
                mapping.insert(
                    *numbers[8]
                        .difference(&six)
                        .next()
                        .expect("Should be the segment e!"),
                    'e',
                );
                numbers[9] = six;
            } else {
                mapping.insert(
                    *numbers[8]
                        .difference(&six)
                        .next()
                        .expect("Should be the segment d!"),
                    'd',
                );
                numbers[0] = six;
            }
        }

        // find g (non-overlap between 9 and 4 that isn't a)
        let a = mapping
            .iter()
            .find(|(_key, value)| *value == &'a')
            .expect("Should have found a already!")
            .0;
        mapping.insert(
            *numbers[9]
                .difference(&numbers[4])
                .find(|x| *x != a)
                .expect("Should be the segment g!"),
            'g',
        );

        // find f (only segment in 7 not yet identified) and b (last unidentified segment)
        let found_segments: HashSet<char> = HashSet::from_iter(mapping.keys().copied());
        mapping.insert(
            *numbers[7]
                .difference(&found_segments)
                .next()
                .expect("Should be the segment f!"),
            'f',
        );
        let found_segments: HashSet<char> = HashSet::from_iter(mapping.keys().copied());
        mapping.insert(
            *numbers[8]
                .difference(&found_segments)
                .next()
                .expect("Should be the segment b!"),
            'b',
        );

        // find 2, 3 and 5
        for five in fives.into_iter() {
            let translation: HashSet<char> = HashSet::from_iter(
                five.iter()
                    .map(|x| *mapping.get(x).expect("Should be valid character!")),
            );
            let num = DISPLAY_NUMBERS
                .iter()
                .position(|n| HashSet::from_iter(n.chars()) == translation)
                .expect("Should find number!");
            numbers[num] = five;
        }

        // find digits
        let mut digits: Vec<usize> = self
            .output
            .iter()
            .map(|x| -> HashSet<char> {
                HashSet::from_iter(
                    x.chars()
                        .map(|x| *mapping.get(&x).expect("Should be valid character!")),
                )
            })
            .map(|y| {
                DISPLAY_NUMBERS
                    .iter()
                    .position(|z| HashSet::from_iter(z.chars()) == y)
                    .expect("Should be valid number!")
            })
            .collect();

        // find value
        let mut value = 0;
        let mut tens = 1;
        while !digits.is_empty() {
            value += digits.pop().unwrap() * tens;
            tens *= 10;
        }

        self._output_value = Some(value);
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Display>> {
    let (input, lines) = separated_list1(newline, parse_display)(input)?;
    Ok((input, lines))
}

fn parse_display(input: &str) -> IResult<&str, Display> {
    let (input, (patterns, _, output)) = tuple((
        separated_list1(nomchar(' '), alpha1),
        tag(" | "),
        separated_list1(nomchar(' '), alpha1),
    ))(input)?;
    let patterns = patterns
        .into_iter()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect_vec();
    let output = output.into_iter().map(|s| s.to_string()).collect_vec();
    Ok((
        input,
        Display {
            patterns,
            output,
            _output_value: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let result = process_part1(input);
        assert_eq!(result, "26");
    }

    #[test]
    fn part2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let result = process_part2(input);
        assert_eq!(result, "61229");
    }
}
