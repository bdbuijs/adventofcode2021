use nom::{
    character::complete::char as nomchar, character::complete::digit1, multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, mut crabs) = parse_input(input).unwrap();
    let med = median(&mut crabs);
    let fuel_cost: i32 = crabs.iter().map(|&x| (x - med).abs()).sum();
    fuel_cost.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, crabs) = parse_input(input).unwrap();
    let mut previous_cost = i32::MAX;
    for mn in 0..1936 {
        let fuel_cost = crabs
            .iter()
            .map(|x| {
                let d = (x - mn).abs();
                (d * (d + 1)) / 2
            })
            .sum();
        if fuel_cost > previous_cost {
            break;
        }
        previous_cost = fuel_cost;
    }
    previous_cost.to_string()
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, lines) = separated_list1(nomchar(','), parse_i32)(input)?;
    Ok((input, lines))
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, digits) = digit1(input)?;
    Ok((input, digits.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = process_part1(input);
        assert_eq!(result, "37");
    }

    #[test]
    fn part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = process_part2(input);
        assert_eq!(result, "168");
    }
}
