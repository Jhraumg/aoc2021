use crate::day18::FishNumberPart::{Complex, Simple};
use anyhow::{anyhow, Error};
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Deref;

use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
enum FishNumberPart {
    Simple(usize),
    Complex(FishNumber),
}

impl FishNumberPart {
    fn to_colored_string(&self, depth: usize) -> String {
        match self {
            Simple(l) => {
                let base = l.to_string();
                if *l >= 10 {
                    base.blue().to_string()
                } else {
                    base
                }
            }
            Complex(fnb) => fnb.to_colored_string(depth + 1),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Simple(l) => *l,
            Complex(c) => c.magnitude(),
        }
    }

    fn try_read(input: &mut dyn Iterator<Item = &str>) -> Result<Self, Error> {
        if let Some(start) = input.next() {
            match start {
                s if s.chars().next().unwrap().is_numeric() => Ok(Simple(
                    s.split(|c| [',', ']'].contains(&c))
                        .next()
                        .unwrap()
                        .parse::<usize>()?,
                )),
                "[" => Ok(Complex(FishNumber {
                    left: Box::new(FishNumberPart::try_read(input)?),
                    right: Box::new(FishNumberPart::try_read(input)?), // let's hope evaluation is not lazy
                })),
                "," | "]" => Self::try_read(input),
                _ => Err(anyhow!("'{}' is not a valid FishNumber start", start)),
            }
        } else {
            Err(anyhow!("empty start ?"))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FishNumber {
    left: Box<FishNumberPart>,
    right: Box<FishNumberPart>,
}
impl std::ops::Add<&FishNumber> for &FishNumber {
    type Output = FishNumber;

    fn add(self, rhs: &FishNumber) -> Self::Output {
        Self::Output {
            left: Box::new(Complex(self.clone())),
            right: Box::new(Complex(rhs.clone())),
        }
        .reduce()
    }
}

impl std::ops::Add<FishNumber> for FishNumber {
    type Output = FishNumber;

    fn add(self, rhs: FishNumber) -> Self::Output {
        Self::Output {
            left: Box::new(Complex(self)),
            right: Box::new(Complex(rhs)),
        }
        .reduce()
    }
}
impl Sum for FishNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result: Option<Self> = None;
        for n in iter {
            result = match result {
                None => Some(n),
                Some(r) => Some(r + n),
            }
        }
        result.unwrap()
    }
}

impl Display for FishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_colored_string(0))
    }
}

impl FishNumber {
    fn to_colored_string(&self, depth: usize) -> String {
        let mut result = format!(
            "[{},{}]",
            self.left.to_colored_string(depth),
            self.right.to_colored_string(depth)
        );
        if depth >= 4 {
            if let (&Simple(_), &Simple(_)) = (&*self.left, &*self.right) {
                result = result.red().to_string();
            }
        }
        result
    }

    fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    fn try_read(input: &str) -> Result<Self, Error> {
        let mut parts = input
            .trim()
            .split_inclusive(|c| ['[', ',', ']'].contains(&c));
        if let Complex(p) = FishNumberPart::try_read(&mut parts)? {
            Ok(p)
        } else {
            Err(anyhow!("'{}' is not a valid Complex FishNumber", input))
        }
    }
}
impl FishNumber {
    fn to_vect(fnb: &FishNumber, depth: usize, vec: &mut Vec<(usize, usize)>) {
        match fnb.left.deref() {
            Simple(l) => vec.push((*l, depth)),
            Complex(p) => {
                Self::to_vect(p, depth + 1, vec);
            }
        }
        match fnb.right.deref() {
            Simple(r) => vec.push((*r, depth)),
            Complex(p) => {
                Self::to_vect(p, depth + 1, vec);
            }
        }
    }

    fn from_vect(vec: &[(usize, usize)]) -> Self {
        let mut vals = vec.iter().copied();
        let current = vals.next().unwrap();
        Self::from_vect_it(current, &mut vals, 0)
    }

    fn from_vect_it(
        current: (usize, usize),
        nexts: &mut dyn Iterator<Item = (usize, usize)>,
        depth: usize,
    ) -> Self {
        let left = Box::new(FishNumberPart::from_vect_it(current, nexts, depth));

        let current = nexts.next().unwrap();
        let right = Box::new(FishNumberPart::from_vect_it(current, nexts, depth));
        Self { left, right }
    }
    fn reduce(&self) -> Self {
        let mut reduce = vec![];
        Self::to_vect(self, 0, &mut reduce);

        let mut conted = true;
        while conted {
            conted = false;
            let mut previous_d: usize = reduce[0].1;
            let len = reduce.len();

            // eprintln!("reducing {}", Self::from_vect(&reduce));
            // searching for xplosion
            for i in 1..len {
                let current = reduce[i];

                if previous_d == current.1 && previous_d >= 4 {
                    // xplosion
                    conted = true;
                    if i > 1 {
                        reduce[i - 2].0 += reduce[i - 1].0;
                    }
                    reduce[i - 1] = (0, previous_d - 1);

                    if i < reduce.len() - 1 {
                        reduce[i + 1].0 += current.0;
                    }
                    reduce.remove(i);

                    break;
                }
                previous_d = current.1;
            }
            if !conted {
                // searching for split
                for i in 0..len {
                    let current = reduce[i];

                    if current.0 >= 10 {
                        conted = true;
                        reduce[i] = (current.0 >> 1, current.1 + 1);
                        reduce.insert(i + 1, ((current.0 >> 1) + current.0 % 2, current.1 + 1));
                        break;
                    }
                }
            }
        }
        Self::from_vect(&reduce)
    }
}
impl FishNumberPart {
    fn from_vect_it(
        current: (usize, usize),
        nexts: &mut dyn Iterator<Item = (usize, usize)>,
        depth: usize,
    ) -> Self {
        match current.1 {
            d if d == depth => Simple(current.0),
            d if d < depth => {
                panic!("unexpected depth {} compared to {}", d, depth);
            }
            _ => Complex(FishNumber::from_vect_it(current, nexts, depth + 1)),
        }
    }
}

fn get_max_magnitude_from_addition(numbers: &[FishNumber]) -> usize {
    let mut max_mag = 0;
    let len = numbers.len();
    for i in 0..len {
        for j in 0..len {
            if i != j {
                let mag = (numbers[i].clone() + numbers[j].clone()).magnitude();
                if mag > max_mag {
                    max_mag = mag;
                }
            }
        }
    }
    max_mag
}

pub fn display_additions() {
    let input = include_str!("../resources/day18_fish_numbers.txt");
    let numbers: Vec<_> = input
        .lines()
        .filter_map(|l| FishNumber::try_read(l).ok())
        .collect();

    let max_magnitude = get_max_magnitude_from_addition(&numbers);
    let sum = numbers.into_iter().sum::<FishNumber>();

    println!("FishNumber sum magnitude: {}", sum.magnitude());
    println!(
        "FishNumber single addition max magnitude: {}",
        max_magnitude
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colored() {
        println!("{}", "this is blue".blue());
        println!("{}", "this is red".red());
    }

    #[test]
    fn aoc_examples_work() {
        for l in ["[0,2]", "[[1,2],3]", "[[1,2],[3,4]"] {
            println!(" l {}", l);
            let nb = FishNumber::try_read(l).unwrap();
            dbg!(nb);
        }
    }
    #[test]
    fn fishnum_can_be_reduced() {
        let fnb = FishNumber::try_read("[[[[[9,8],1],2],3],4]").unwrap();
        assert_eq!("[[[[0,9],2],3],4]", fnb.reduce().to_string());

        assert_eq!(
            "[[[[0,0],0],0],0]",
            FishNumber::try_read("[[[[[11,0],0],0],0],0]")
                .unwrap()
                .reduce()
                .to_string()
        );
        assert_eq!(
            "[[[[0,0],[5,6]],0],0]",
            FishNumber::try_read("[[[[[0,0],0],11],0],0]")
                .unwrap()
                .reduce()
                .to_string()
        );
    }

    #[test]
    fn sum_from_aoc_example_match() {
        let fnb1 = FishNumber::try_read(" [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap();
        let fnb2 = FishNumber::try_read("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap();
        let fnb3 = fnb1 + fnb2;
        assert_eq!(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            fnb3.to_string()
        );

        let inputs = [
            (
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
                "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]",
            ),
            (
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
                "[1,1]
[2,2]
[3,3]
[4,4]",
            ),
            (
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
            ),
            (
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            ),
        ];
        for (result, sum) in inputs {
            assert_eq!(
                result,
                sum.lines()
                    .filter_map(|l| FishNumber::try_read(l).ok())
                    .sum::<FishNumber>()
                    .to_string()
            );
        }

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

        let sum = input
            .lines()
            .filter_map(|l| FishNumber::try_read(l).ok())
            .sum::<FishNumber>();
        println!("******************************");
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            sum.to_string()
        );
    }

    #[test]
    fn aoc_magnitude_example_works() {
        let inputs = [
            (143, "[[1,2],[[3,4],5]]"),
            (1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]] "),
            (445, "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
            (791, "[[[[3,0],[5,3]],[4,4]],[5,5]]"),
            (1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]"),
            (
                3488,
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ];
        for (m, fnb) in &inputs {
            assert_eq!(*m, FishNumber::try_read(fnb).unwrap().magnitude());
        }

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
        let numbers: Vec<_> = input
            .lines()
            .filter_map(|l| FishNumber::try_read(l).ok())
            .collect();

        let sum = numbers.iter().cloned().sum::<FishNumber>();
        assert_eq!(4140, sum.magnitude());

        assert_eq!(3993, get_max_magnitude_from_addition(&numbers));
    }
}
