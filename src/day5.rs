use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Directions {
    HorzVert,
    HorzVertDiag,
}
impl Vent {
    pub fn max(&self) -> usize {
        max(max(self.x1, self.x2), max(self.y1, self.y2))
    }

    // Could do try_new(&str) -> Option<Vent>
    pub fn new(line: &str) -> Self {
        if let Some((x1, y1, x2, y2)) = line
            .split("->")
            .flat_map(|coords| coords.split(','))
            .map(|val| val.trim().parse::<usize>().unwrap())
            .collect_tuple()
        {
            return Vent { x1, y1, x2, y2 };
        }
        panic!("could not parse a Vent from {}", line)
    }
    pub fn points(&self, dir: Directions) -> Vec<Point> {
        if self.x1 == self.x2 {
            // vertical
            return (min(self.y1, self.y2)..max(self.y1, self.y2) + 1)
                .map(|v| Point { x: self.x1, y: v })
                .collect();
        } else if self.y1 == self.y2 {
            // horizontal
            return (min(self.x1, self.x2)..max(self.x1, self.x2) + 1)
                .map(|v| Point { x: v, y: self.y1 })
                .collect();
        } else if dir == Directions::HorzVertDiag && {
            // abs for usize...
            let dx = if self.x1 < self.x2 {
                self.x2 - self.x1
            } else {
                self.x1 - self.x2
            };
            let dy = if self.y1 < self.y2 {
                self.y2 - self.y1
            } else {
                self.y1 - self.y2
            };
            dx == dy
        } {
            return (0..max(self.x1, self.x2) - min(self.x1, self.x2) + 1)
                .map(|i| {
                    let x = if self.x1 < self.x2 {
                        self.x1 + i
                    } else {
                        self.x1 - i
                    };
                    let y = if self.y1 < self.y2 {
                        self.y1 + i
                    } else {
                        self.y1 - i
                    };
                    Point { x, y }
                })
                .collect();
        }
        eprintln!("unknown direction for {:?}", self);
        vec![]
    }
}

fn parse_vents(vents: &str) -> Vec<Vent> {
    vents.lines().map(Vent::new).collect_vec()
}

fn count_overlapped_more_than_twice(vents: &str, dir: Directions) -> usize {
    let vents = parse_vents(vents);
    let map_size = vents.iter().map(|v| v.max()).max().unwrap() + 1;

    let mut map = vec![vec![0usize; map_size]; map_size];

    // lets mark map for each vent points
    for vent in vents {
        for Point { x, y } in vent.points(dir) {
            map[x][y] += 1;
        }
    }

    // filter values above 1
    map.iter()
        .flat_map(|l| l.iter())
        .filter(|&&v| v > 1)
        .count()
}

pub fn print_hydrothermals() {
    let vents = include_str!("day5_hydrothermal_vents.txt");
    println!(
        "considering horizontal and vertical lines only, {} places are overlapsed more than once",
        count_overlapped_more_than_twice(vents, Directions::HorzVert)
    );
    println!("considering horizontal, vertical and diagonal lines, {} places are overlapsed more than once", count_overlapped_more_than_twice(vents, Directions::HorzVertDiag));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let vents = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(
            5,
            count_overlapped_more_than_twice(vents, Directions::HorzVert)
        );
        assert_eq!(
            12,
            count_overlapped_more_than_twice(vents, Directions::HorzVertDiag)
        );
    }
}
