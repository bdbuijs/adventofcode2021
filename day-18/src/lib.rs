use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use itertools::Itertools;

use nom::{
    character::complete::newline,
    character::complete::{char as nomchar, digit1},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (rem, nums) = parse_input(input).unwrap();
    assert!(rem.is_empty());
    let sum = sum_nums(nums);
    let s = sum.borrow().magnitude().to_string();
    s
}

pub fn process_part2(input: &str) -> String {
    let (rem, nums) = parse_input(input).unwrap();
    assert!(rem.is_empty());
    let num_strings: Vec<_> = nums
        .into_iter()
        .map(|num| num.borrow().to_string())
        .collect();
    let max = num_strings
        .into_iter()
        .permutations(2)
        .map(|perm| {
            let (first, second) = (perm.first().unwrap(), perm.last().unwrap());
            let s = format!("{}\n{}", first, second);
            let (rem, nums) = parse_input(&s).unwrap();
            assert!(rem.is_empty());
            sum_nums(nums).borrow().magnitude()
        })
        .max()
        .unwrap();
    max.to_string()
}

/// Sums all Nums one by one, reducing at every step
fn sum_nums(nums: Vec<Rc<RefCell<Num>>>) -> Rc<RefCell<Num>> {
    let mut it = nums.into_iter();
    let first = it.next().unwrap();
    it.fold(first, |acc, el| {
        let current = Rc::new(RefCell::new(Num::Pair(acc, el, None)));
        current.borrow_mut().connect(None, current.clone());
        reduce(current)
    })
}

#[derive(Clone, PartialEq, Eq)]
enum Num {
    /// (left, right, parent)
    Pair(Rc<RefCell<Num>>, Rc<RefCell<Num>>, Option<Rc<RefCell<Num>>>),
    /// (value, parent)
    Single(u8, Option<Rc<RefCell<Num>>>),
}

impl Num {
    /// Returns numeric value of Single Num, panics if called on Pair
    fn value(&self) -> u8 {
        match self {
            Self::Pair(_, _, _) => panic!("Attempted to get value of pair!"),
            Self::Single(val, _) => *val,
        }
    }

    /// Returns left child of Num
    fn left(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            Self::Single(_, _) => None,
            Self::Pair(left, _, _) => Some(Rc::clone(left)),
        }
    }

    /// Returns right child of Num
    fn right(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            Self::Single(_, _) => None,
            Self::Pair(_, right, _) => Some(Rc::clone(right)),
        }
    }

    /// Returns parent of Num
    fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            Self::Single(_, p) => p.clone(),
            Self::Pair(_, _, p) => p.clone(),
        }
    }

    /// Connects Nums recursively (used only in parsing functions, could be refactored out)
    fn connect(&mut self, parent: Option<Rc<RefCell<Self>>>, rc: Rc<RefCell<Self>>) {
        match self {
            Self::Single(_, opt) => {
                if let Some(parent) = parent {
                    let _ = opt.insert(parent);
                }
            }
            Self::Pair(left, right, opt) => {
                if let Some(parent) = parent {
                    let _ = opt.insert(parent);
                }
                left.borrow_mut()
                    .connect(Some(Rc::clone(&rc)), Rc::clone(left));
                right
                    .borrow_mut()
                    .connect(Some(Rc::clone(&rc)), Rc::clone(right));
            }
        }
    }

    /// Set parent of Num
    fn set_parent(&mut self, parent: Rc<RefCell<Self>>) {
        match self {
            Self::Single(_, ref mut p) => {
                *p = Some(parent);
            }
            Self::Pair(_, _, ref mut p) => {
                *p = Some(parent);
            }
        }
    }

    /// Split Single Num into Pair (if needed, panics if called on Pair)
    fn split(&mut self) {
        let clone = self.clone();
        match clone {
            Self::Single(val, parent) => {
                if val >= 10 {
                    let new_left_val = val / 2;
                    let new_right_val = (val + 1) / 2;
                    let new_self = Self::Pair(
                        Rc::new(RefCell::new(Num::Single(new_left_val, None))),
                        Rc::new(RefCell::new(Num::Single(new_right_val, None))),
                        parent,
                    );
                    *self = new_self;
                }
            }
            _ => panic!("Called split on a pair!"),
        }
    }

    /// Returns the calculated magnitude of the Num
    fn magnitude(&self) -> usize {
        match self {
            Self::Single(n, _) => *n as usize,
            Self::Pair(left, right, _) => {
                left.borrow().magnitude() * 3 + right.borrow().magnitude() * 2
            }
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(n, _) => write!(f, "{}", n),
            Self::Pair(left, right, _) => {
                write!(f, "[{},{}]", left.borrow(), right.borrow())
            }
        }
    }
}

/// Recursively set parents of all child nodes from node
fn connect(node: Rc<RefCell<Num>>) {
    let parent_clone = node.borrow().clone();
    match parent_clone {
        Num::Pair(left, right, _) => {
            left.borrow_mut().set_parent(node.clone());
            right.borrow_mut().set_parent(node);
            connect(left);
            connect(right);
        }
        Num::Single(_, _) => { /* no further children to connect */ }
    }
}

/// Explode all numbers in the tree that should explode, in left-dfs order
fn explode(tree: Rc<RefCell<Num>>) {
    while let Some(node) = should_explode(tree.clone()) {
        explode_node(node.clone());
    }
}

/// Returns leftmost node that should explode
fn should_explode(tree: Rc<RefCell<Num>>) -> Option<Rc<RefCell<Num>>> {
    let mut stack = VecDeque::new();
    stack.push_back((tree, 0));
    while let Some((node, depth)) = stack.pop_back() {
        match node.borrow().clone() {
            Num::Single(_, _) => continue,
            Num::Pair(left, right, _) => {
                if depth > 3 {
                    if let (Num::Single(_, _), Num::Single(_, _)) =
                        (left.borrow().clone(), right.borrow().clone())
                    {
                        return Some(node.clone());
                    }
                }
                stack.push_back((right, depth + 1));
                stack.push_back((left, depth + 1));
            }
        }
    }
    None
}

/// Explode node
fn explode_node(node: Rc<RefCell<Num>>) {
    let new_num: Option<Num>;
    let old_num = node.borrow().clone();
    match old_num {
        Num::Single(_, _) => panic!("Can't explode a single num!"),
        Num::Pair(left, right, parent) => {
            let left_val = left.borrow().value();
            let right_val = right.borrow().value();
            new_num = Some(Num::Single(0, parent));
            if let Some(num_to_left) = num_to_left(node.clone()) {
                let new_left_val = left_val + num_to_left.borrow().value();
                let new_left_parent = num_to_left.borrow().parent();
                *num_to_left.borrow_mut() = Num::Single(new_left_val, new_left_parent);
            }
            if let Some(num_to_right) = num_to_right(node.clone()) {
                let new_right_val = right_val + num_to_right.borrow().value();
                let new_right_parent = num_to_right.borrow().parent();
                *num_to_right.borrow_mut() = Num::Single(new_right_val, new_right_parent);
            }
        }
    }
    if let Some(num) = new_num {
        *node.borrow_mut() = num;
    }
}

/// Find number immediately to the left of the given node
fn num_to_left(node: Rc<RefCell<Num>>) -> Option<Rc<RefCell<Num>>> {
    if let Some(mut parent) = node.borrow().parent() {
        if Rc::ptr_eq(&node, &parent.borrow().left().unwrap()) {
            // I'm on the left, go up until I find a node on the right, then go down the left side of its parent, keeping right until I get to a Single
            let mut grandparent = parent.borrow().parent();

            while let Some(top) = grandparent {
                if Rc::ptr_eq(&parent, &top.borrow().left().unwrap()) {
                    // still on the left
                    parent = top.clone();
                    grandparent = top.borrow().parent();
                } else {
                    // on the right, now go down the left side of the parent and keep right until Single is found
                    grandparent = None;
                    let mut potential_single = top.borrow().left();
                    while let Some(node_rc) = potential_single {
                        let node = node_rc.borrow().clone();
                        match node {
                            Num::Single(_, _) => {
                                return Some(node_rc);
                            }
                            Num::Pair(_left, right, _) => {
                                potential_single = Some(right);
                            }
                        }
                    }
                }
            }
        } else {
            // I'm on the right, go down left child of parent until there's a Single on the right
            let mut current = parent.borrow().left();
            while let Some(node) = current {
                let check = node.borrow().clone();
                match check {
                    Num::Single(_, _) => return Some(node),
                    Num::Pair(_, right, _) => {
                        current = Some(right);
                    }
                }
            }
        }
    }

    None
}

/// Find number immediately to the right of the given node
fn num_to_right(node: Rc<RefCell<Num>>) -> Option<Rc<RefCell<Num>>> {
    if let Some(mut parent) = node.borrow().parent() {
        if Rc::ptr_eq(&node, &parent.borrow().right().unwrap()) {
            // I'm on the right, go up until I find a node on the right, then go down the right side of its parent, keeping left until I get to a Single
            let mut grandparent = parent.borrow().parent();

            while let Some(top) = grandparent {
                if Rc::ptr_eq(&parent, &top.borrow().right().unwrap()) {
                    // still on the right
                    parent = top.clone();
                    grandparent = top.borrow().parent();
                } else {
                    // on the left, now go down the right side of the parent and keep left until Single is found
                    grandparent = None;
                    let mut potential_single = top.borrow().right();
                    while let Some(node_rc) = potential_single {
                        let node = node_rc.borrow().clone();
                        match node {
                            Num::Single(_, _) => {
                                return Some(node_rc);
                            }
                            Num::Pair(left, _right, _) => {
                                potential_single = Some(left);
                            }
                        }
                    }
                }
            }
        } else {
            // I'm on the left, go down right child of parent until there's a Single on the left
            let mut current = parent.borrow().right();
            while let Some(node) = current {
                let check = node.borrow().clone();
                match check {
                    Num::Single(_, _) => return Some(node),
                    Num::Pair(left, _, _) => {
                        current = Some(left);
                    }
                }
            }
        }
    }
    None
}

/// Find the leftmost number that should be split and split it
fn split(tree: Rc<RefCell<Num>>) {
    if let Some(node) = should_split(tree) {
        node.borrow_mut().split();
        connect(node);
    }
}

/// Returns leftmost node that should be split
fn should_split(tree: Rc<RefCell<Num>>) -> Option<Rc<RefCell<Num>>> {
    let mut stack = VecDeque::new();
    stack.push_back((tree, 0));
    while let Some((node, depth)) = stack.pop_back() {
        match node.borrow().clone() {
            Num::Single(val, _) => {
                if val >= 10 {
                    return Some(node.clone());
                }
            }
            Num::Pair(left, right, _) => {
                stack.push_back((right, depth + 1));
                stack.push_back((left, depth + 1));
            }
        }
    }
    None
}

/// Reduces number (explodes and splits until fully reduced)
fn reduce(tree: Rc<RefCell<Num>>) -> Rc<RefCell<Num>> {
    let mut tree_string = tree.borrow().to_string();
    loop {
        explode(tree.clone());
        split(tree.clone());
        let new_tree_string = tree.borrow().to_string();
        if new_tree_string == tree_string {
            break tree;
        }
        tree_string = new_tree_string;
    }
}

type Line = Rc<RefCell<Num>>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = parse_num(input)?;
    let rc = Rc::new(RefCell::new(line));
    connect(rc.clone());
    Ok((input, rc))
}

fn parse_num(input: &str) -> IResult<&str, Num> {
    let (input, _) = nomchar('[')(input)?;

    let (input, left) = if let Ok((input, digits)) = digit1::<_, ()>(input) {
        let n = digits.parse::<u8>().unwrap();
        (input, Rc::new(RefCell::new(Num::Single(n, None))))
    } else {
        let (input, num) = parse_num(input)?;
        (input, Rc::new(RefCell::new(num)))
    };

    let (input, _) = nomchar(',')(input)?;

    let (input, right) = if let Ok((input, digits)) = digit1::<_, ()>(input) {
        let n = digits.parse::<u8>().unwrap();
        (input, Rc::new(RefCell::new(Num::Single(n, None))))
    } else {
        let (input, num) = parse_num(input)?;
        (input, Rc::new(RefCell::new(num)))
    };

    let (input, _) = nomchar(']')(input)?;

    let num = Num::Pair(left, right, None);
    Ok((input, num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbg() {
        let (_, num) = parse_line("[[[[[9,8],1],2],3],4]").unwrap();
        dbg!(num.borrow().to_string());
    }

    #[test]
    fn magnitude() {
        let (_, num) = parse_line("[9,1]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "29");
        let (_, num) = parse_line("[1,9]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "21");
        let (_, num) = parse_line("[[9,1],[1,9]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "129");
        let (_, num) = parse_line("[[1,2],[[3,4],5]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "143");
        let (_, num) = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "1384");
        let (_, num) = parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "445");
        let (_, num) = parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "791");
        let (_, num) = parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "1137");
        let (_, num) = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(format!("{}", num.borrow().magnitude()), "3488");
    }

    #[test]
    fn test_split() {
        let num = Rc::new(RefCell::new(Num::Single(10, None)));
        split(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[5,5]");
        let num = Rc::new(RefCell::new(Num::Single(11, None)));
        split(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[5,6]");
        let num = Rc::new(RefCell::new(Num::Single(12, None)));
        split(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[6,6]");
    }

    #[test]
    fn test_explode_node() {
        let (_, num) = parse_line("[[[[[9,8],1],2],3],4]").unwrap();
        explode_node(
            num.borrow()
                .left()
                .unwrap()
                .borrow()
                .left()
                .unwrap()
                .borrow()
                .left()
                .unwrap()
                .borrow()
                .left()
                .unwrap(),
        );
        assert_eq!(format!("{}", num.borrow()), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_explode() {
        let (_, num) = parse_line("[[[[[9,8],1],2],3],4]").unwrap();
        explode(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[[[[0,9],2],3],4]");

        let (_, num) = parse_line("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        explode(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[7,[6,[5,[7,0]]]]");

        let (_, num) = parse_line("[[6,[5,[4,[3,2]]]],1]").unwrap();
        explode(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[[6,[5,[7,0]]],3]");

        let (_, num) = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        explode(num.clone());
        assert_eq!(format!("{}", num.borrow()), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_reduce() {
        let (_, num) = parse_line("[11,12]").unwrap();
        let num = reduce(num);
        assert_eq!(format!("{}", num.borrow()), "[[5,6],[6,6]]");
    }

    #[test]
    fn sum0() {
        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";
        let (_, nums) = parse_input(input).unwrap();
        let sum = sum_nums(nums);
        assert_eq!(
            format!("{}", sum.borrow()),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn sum1() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]";
        let (_, nums) = parse_input(input).unwrap();
        let sum = sum_nums(nums);
        assert_eq!(format!("{}", sum.borrow()), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn sum2() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
        let (_, nums) = parse_input(input).unwrap();
        let sum = sum_nums(nums);
        assert_eq!(format!("{}", sum.borrow()), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn sum3() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        let (_, nums) = parse_input(input).unwrap();
        let sum = sum_nums(nums);
        assert_eq!(format!("{}", sum.borrow()), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn sum4() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let (_, nums) = parse_input(input).unwrap();
        let sum = sum_nums(nums);
        assert_eq!(
            format!("{}", sum.borrow()),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn part1() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = process_part1(input);
        assert_eq!(result, "4140");
    }

    #[test]
    fn part2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = process_part2(input);
        assert_eq!(result, "3993");
    }
}
