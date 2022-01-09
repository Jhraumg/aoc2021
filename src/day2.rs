use crate::day2::Movement::{Down, Forward, Up};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    horz: usize,
    depth: usize,
    aim: usize,
}

#[derive(Debug)]
enum Movement {
    Forward(usize),
    Down(usize),
    Up(usize),
}
impl Position {
    pub fn new() -> Position {
        Position {
            horz: 0,
            depth: 0,
            aim: 0,
        }
    }
    pub fn apply_mvt(&mut self, mvt: Movement) {
        match mvt {
            Forward(val) => self.horz += val,
            Down(val) => self.depth += val,
            Up(val) => self.depth -= val,
        }
    }

    pub fn apply_aimed_mvt(&mut self, mvt: Movement) {
        match mvt {
            Forward(val) => {
                self.horz += val;
                self.depth += self.aim * val
            }
            Down(val) => self.aim += val,
            Up(val) => self.aim -= val,
        }
    }
}

fn parse_movement(line: &str) -> Option<Movement> {
    let instructions: Option<(&str, &str)> = line.split_whitespace().collect_tuple();
    instructions
        .map(|(direct, val)| {
            val.parse::<usize>()
                .ok()
                .map(|val| match direct {
                    "forward" => Some(Forward(val)),
                    "down" => Some(Down(val)),
                    "up" => Some(Up(val)),
                    _ => None,
                })
                .flatten()
        })
        .flatten()
}

fn apply_plan(init_pos: &Position, plan: &str) -> Position {
    let movements: Vec<_> = plan.lines().filter_map(parse_movement).collect();
    let mut target = *init_pos;
    for mvt in movements {
        target.apply_mvt(mvt);
    }
    target
}

fn apply_aimed_plan(init_pos: &Position, plan: &str) -> Position {
    let movements: Vec<_> = plan.lines().filter_map(parse_movement).collect();
    let mut target = *init_pos;
    for mvt in movements {
        target.apply_aimed_mvt(mvt);
    }
    target
}

pub fn print_position() {
    let plan = include_str!("../resources/day2_movements.txt");
    let origin = Position::new();
    let target = apply_plan(&origin, plan);
    println!("target_position : {:?}", target);
    println!("product : {:?}", target.horz * target.depth);

    let aimed_target = apply_aimed_plan(&origin, plan);
    println!("target_position : {:?}", aimed_target);
    println!("product : {:?}", aimed_target.horz * aimed_target.depth);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc_examples_work() {
        let init_pos = Position::new();
        let plan = "forward 5
down 5
forward 8
up 3
down 8
forward 2
        ";
        assert_eq!(
            Position {
                horz: 15,
                depth: 10,
                aim: 0
            },
            apply_plan(&init_pos, plan)
        );

        let aimed_pos = apply_aimed_plan(&init_pos, plan);
        assert_eq!(15, aimed_pos.horz);
        assert_eq!(60, aimed_pos.depth);
    }
}
