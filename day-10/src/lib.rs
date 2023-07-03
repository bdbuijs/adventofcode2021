use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, chars) = parse_input(input).unwrap();
    let mut syntax_error_score = 0;
    let mut autocomplete_scores = Vec::new();
    for line in chars.into_iter() {
        let mut corrupted = false;
        let mut stack = Vec::new();
        for c in line.into_iter() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '(' {
                        corrupted = true;
                        syntax_error_score += 3;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '{' {
                        corrupted = true;
                        syntax_error_score += 1197;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '[' {
                        corrupted = true;
                        syntax_error_score += 57;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '<' {
                        corrupted = true;
                        syntax_error_score += 25137;
                        break;
                    }
                }
                _ => panic!("Didn't expect character '{}'", c),
            }
        }
        if corrupted {
            continue;
        }
        let mut autocomplete_score = 0_usize;
        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            autocomplete_score *= 5;
            match c {
                '(' => autocomplete_score += 1,
                '{' => autocomplete_score += 3,
                '[' => autocomplete_score += 2,
                '<' => autocomplete_score += 4,
                _ => panic!("Didn't expect character '{}'", c),
            }
        }
        autocomplete_scores.push(autocomplete_score);
    }

    syntax_error_score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, chars) = parse_input(input).unwrap();
    let mut autocomplete_scores = Vec::new();
    for line in chars.into_iter() {
        let mut corrupted = false;
        let mut stack = Vec::new();
        for c in line.into_iter() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '(' {
                        corrupted = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '{' {
                        corrupted = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '[' {
                        corrupted = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().expect("Stack shouldn't get empty!") != '<' {
                        corrupted = true;
                        break;
                    }
                }
                _ => panic!("Didn't expect character '{}'", c),
            }
        }
        if corrupted {
            continue;
        }
        let mut autocomplete_score = 0_usize;
        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            autocomplete_score *= 5;
            match c {
                '(' => autocomplete_score += 1,
                '{' => autocomplete_score += 3,
                '[' => autocomplete_score += 2,
                '<' => autocomplete_score += 4,
                _ => panic!("Didn't expect character '{}'", c),
            }
        }
        autocomplete_scores.push(autocomplete_score);
    }
    autocomplete_scores.sort();
    autocomplete_scores[autocomplete_scores.len() / 2].to_string()
}

type Line = Vec<char>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(one_of(r"(){}[]<>"))(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let result = process_part1(input);
        assert_eq!(result, "26397");
    }

    #[test]
    fn part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let result = process_part2(input);
        assert_eq!(result, "288957");
    }
}
