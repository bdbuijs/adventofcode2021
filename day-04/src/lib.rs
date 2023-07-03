use nom::{
    character::complete::char as nomchar,
    character::complete::{digit1, newline, space0, space1},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (draws, mut boards)) = parse_input(input).unwrap();
    assert_eq!(input, "");
    for &draw in draws.iter() {
        let mut new_boards = Vec::new();
        for board in boards.into_iter() {
            let mut new_board = Vec::new();
            for row in board.into_iter() {
                let new_row: Vec<usize> = row
                    .into_iter()
                    .map(|x| if x == draw { 100 } else { x })
                    .collect();
                new_board.push(new_row);
            }
            if bingo(&new_board) {
                let score: usize = new_board
                    .into_iter()
                    .flatten()
                    .filter(|&x| x != 100)
                    .sum::<usize>()
                    * draw;
                return score.to_string();
            }
            new_boards.push(new_board);
        }
        boards = new_boards;
    }
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (draws, mut boards)) = parse_input(input).unwrap();
    for &draw in draws.iter() {
        let mut new_boards = Vec::new();
        for board in boards.iter() {
            let mut new_board = Vec::new();
            for row in board.iter() {
                let new_row: Vec<usize> = row
                    .iter()
                    .map(|&x| if x == draw { 100 } else { x })
                    .collect();
                new_board.push(new_row);
            }
            if bingo(&new_board) {
                let score: usize = new_board
                    .into_iter()
                    .flatten()
                    .filter(|&x| x != 100)
                    .sum::<usize>()
                    * draw;
                if boards.len() == 1 {
                    return score.to_string();
                }
                continue;
            }
            new_boards.push(new_board);
        }
        boards = new_boards;
    }
    "".to_string()
}

fn bingo(b: &[Vec<usize>]) -> bool {
    let row_bingo: Vec<usize> = vec![100, 100, 100, 100, 100];
    let mut column_bingo = [0; 5];
    for row in b.iter() {
        if row == &row_bingo {
            return true;
        }
        for (i, &col) in row.iter().enumerate() {
            if col == 100 {
                column_bingo[i] += 1;
            }
        }
    }
    for &column in column_bingo.iter() {
        if column == 5 {
            return true;
        }
    }

    false
}

type Draws = Vec<usize>;
type Boards = Vec<Vec<Vec<usize>>>;

fn parse_input(input: &str) -> IResult<&str, (Draws, Boards)> {
    let (input, draws) = terminated(
        separated_list1(nomchar(','), parse_usize),
        pair(newline, newline),
    )(input)?;
    let (input, boards) = separated_list1(pair(newline, newline), parse_board)(input)?;
    Ok((input, (draws, boards)))
}

fn parse_board(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let (input, board) = separated_list1(
        newline,
        preceded(space0, separated_list1(space1, parse_usize)),
    )(input)?;
    Ok((input, board))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, n) = digit1(input)?;
    Ok((input, n.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        let result = process_part1(input);
        assert_eq!(result, "4512");
    }

    #[test]
    fn part2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        let result = process_part2(input);
        assert_eq!(result, "1924");
    }
}
