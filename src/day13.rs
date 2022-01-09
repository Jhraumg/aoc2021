use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

type Points = (usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}
use Fold::*;

impl Fold {
    fn new(input: &str) -> Result<Self> {
        if let Some((t, v)) = input.trim().split('=').collect_tuple() {
            let val: usize = v.parse()?;
            match t {
                "x" => Ok(X(val)),
                "y" => Ok(Y(val)),
                _ => Err(anyhow!("unknown fold type {}", t)),
            }
        } else {
            Err(anyhow!("cannot convert {} to fold", input))
        }
    }
}
struct Paper {
    points: Vec<Points>,
    folds: Vec<Fold>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = *self.points.iter().map(|(x, _)| x).max().unwrap_or(&0);
        let max_y = *self.points.iter().map(|(_, y)| y).max().unwrap_or(&0);
        let mut display = vec![vec![' '; max_x + 1]; max_y + 1];

        f.write_fmt(format_args!("Paper({},{})\n", max_x, max_y))?;

        for (x, y) in &self.points {
            display[*y][*x] = '#';
        }
        for line in &display {
            f.write_fmt(format_args!("{}\n", line.iter().join("")))?;
        }
        Ok(())
    }
}

impl Paper {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines();
        let points: Vec<Points> = lines
            .clone()
            .filter(|l| l.contains(','))
            .filter_map(|l| {
                l.split(',')
                    .filter_map(|sub| sub.parse::<usize>().ok())
                    .collect_tuple()
            })
            .collect();

        let folds = lines
            .clone()
            .filter_map(|l| l.strip_prefix("fold along "))
            .filter_map(|l| Fold::new(l).ok())
            .collect();
        Self { points, folds }
    }

    pub fn fold(&mut self, folds_count: usize) {
        let folding = |val: usize, f_val: usize| match val {
            v if v < f_val => v,
            v if v > f_val && v <= 2 * f_val => 2 * f_val - v,
            v if v == f_val => panic!("value on the fold line"),
            _ => panic!("folding too near the start of the sheet"),
        };

        for fold in self.folds.iter().take(folds_count) {
            for (x, y) in self.points.iter_mut() {
                match fold {
                    X(v) => {
                        *x = folding(*x, *v);
                    }
                    Y(v) => {
                        *y = folding(*y, *v);
                    }
                }
            }
        }
    }

    pub fn count_points(&self) -> usize {
        self.points.iter().unique().count()
    }
}

fn count_dots_after_folding(input: &str, folds_count: usize) -> usize {
    let mut paper = Paper::parse(input);
    paper.fold(folds_count);
    paper.count_points()
}

pub fn print_origami_details() {
    let input = include_str!("../resources/day13_transparent_paper.txt");

    println!(
        "number of dots after 1 fold {}",
        count_dots_after_folding(input, 1)
    );

    let mut paper = Paper::parse(input);
    paper.fold(paper.folds.len());
    println!("{}", paper);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples_work() {
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
        assert_eq!(17, count_dots_after_folding(input, 1))
    }
}
