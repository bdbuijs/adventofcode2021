use nom::{
    bytes::complete::tag,
    character::complete::{char as nomchar, u64 as nomu64},
    character::complete::{newline, one_of},
    multi::separated_list1,
    sequence::{pair, terminated, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, (points, folds)) = parse_input(input).unwrap();
    let mut paper = Paper::new(points);
    paper.fold(folds.first().unwrap());
    paper.count().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (points, folds)) = parse_input(input).unwrap();
    let mut paper = Paper::new(points);
    folds.into_iter().for_each(|fold| paper.fold(&fold));
    paper
        .dots
        .into_iter()
        .map(|row| {
            let mut r = row
                .into_iter()
                .map(|pixel| if pixel { '#' } else { ' ' })
                .collect::<String>();
            r.push('\n');
            r
        })
        .collect::<String>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Vec<bool>>,
}

impl Paper {
    fn new(points: Vec<Point>) -> Self {
        let (xmax, ymax) = points
            .iter()
            .fold((0, 0), |(x, y), el| (x.max(el.x), y.max(el.y)));
        let mut dots = vec![vec![false; xmax + 1]; ymax + 1];
        points.into_iter().for_each(|p| {
            dots[p.y][p.x] = true;
        });
        Self { dots }
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(x) => {
                (1..=((*x).min(self.width() - x))).for_each(|i| {
                    self.dots.iter_mut().for_each(|row| {
                        row[*x - i] |= row[*x + i];
                    });
                });
                self.dots.iter_mut().for_each(|row| row.resize(*x, false));
            }
            Fold::Y(y) => {
                let mut i = 0;
                while i < *y {
                    let bottom_row = self.dots.pop().unwrap();
                    self.dots[i]
                        .iter_mut()
                        .zip(bottom_row)
                        .for_each(|(top, btm)| *top |= btm);
                    i += 1;
                }
                self.dots.pop().unwrap();
            }
        }
    }

    fn count(&self) -> usize {
        self.dots
            .iter()
            .flat_map(|row| row.iter())
            .filter(|p| **p)
            .count()
    }

    fn width(&self) -> usize {
        self.dots.first().unwrap().len()
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Point>, Vec<Fold>)> {
    let (input, points) = terminated(
        separated_list1(newline, parse_point),
        pair(newline, newline),
    )(input)?;
    let (input, folds) = separated_list1(newline, parse_fold)(input)?;
    Ok((input, (points, folds)))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, _, y)) = tuple((nomu64, nomchar(','), nomu64))(input)?;
    let (x, y) = (x as usize, y as usize);
    Ok((input, Point { x, y }))
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    let (input, (_, dir, _, i)) =
        tuple((tag("fold along "), one_of("xy"), nomchar('='), nomu64))(input)?;
    let fold = match dir {
        'x' => Fold::X(i as usize),
        'y' => Fold::Y(i as usize),
        _ => unreachable!(),
    };
    Ok((input, fold))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = process_part1(input);
        assert_eq!(result, "17");
    }

    #[test]
    fn part1_2() {
        let input = "0,0
2,0
3,0
6,0
9,0
1,1
4,1
7,2
10,2
1,3
4,3
1,4
3,4
6,4
8,4
9,4
10,4
0,6

fold along x=5";
        let result = process_part1(input);
        assert_eq!(result, "17");
    }

    #[test]
    fn part2() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = process_part2(input);
        let count = result.chars().filter(|&c| c == '#').count();
        assert_eq!(count, 16);
    }
}
