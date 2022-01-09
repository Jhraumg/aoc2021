use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::cmp::{max, min};
use std::str::FromStr;

struct TargetArea {
    x: (isize, isize),
    y: (isize, isize),
}
impl FromStr for TargetArea {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut x: Option<(isize, isize)> = None;
        let mut y: Option<(isize, isize)> = None;

        if let Some(ranges) = input
            .trim()
            .strip_prefix("target area:")
            .map(|l| l.split(','))
        {
            for r in ranges {
                if let Some((name, vals)) = r.split('=').collect_tuple() {
                    if let Some((left, right)) = vals.split("..").collect_tuple() {
                        let min = left.parse()?;
                        let max = right.parse()?;
                        if name.contains('x') {
                            x = Some((min, max));
                        } else {
                            y = Some((min, max));
                        }
                    }
                }
            }
        }

        if x.and(y).is_some() {
            let x = x.unwrap();
            let y = y.unwrap();
            Ok(TargetArea { x, y })
        } else {
            Err(anyhow!("could not parse {}", input))
        }
    }
}

fn get_highest_position_from_trajectory(
    init_v: (isize, isize),
    target: &TargetArea,
) -> Option<isize> {
    let mut current_pos = (0, 0);
    let mut current_v = init_v;
    let mut highest = 0;

    // vx >=steps(steps-1) /2, donc steps <=sqrt(2vx)+1

    while current_pos.1 > target.y.0 || current_v.0 != 0 {
        current_pos.0 += current_v.0;
        current_pos.1 += current_v.1;
        highest = max(highest, current_pos.1);
        current_v.0 = match current_v.0 {
            0 => 0,
            v if v > 0 => v - 1,
            v => v + 1,
        };
        current_v.1 -= 1;
        if current_pos.1 <= target.y.1
            && current_pos.1 >= target.y.0
            && current_pos.0 >= target.x.0
            && current_pos.0 <= target.x.1
        {
            break;
        }
    }
    if current_pos.1 >= target.y.0 && current_pos.0 >= target.x.0 && current_pos.0 <= target.x.1 {
        Some(highest)
    } else {
        None
    }
}

fn count_valid_trajectories(target: &TargetArea) -> usize {
    let x_range = min(0, target.x.0)..max(0, target.x.1) + 1;
    let y_range = target.y.0..10 * max(target.y.0.abs(), target.y.1.abs()); // FIXME : WTF Shanon law :-p
    x_range
        .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
        .filter(|(x, y)| get_highest_position_from_trajectory((*x, *y), target).is_some())
        .count()
}

fn find_highest_position(target: &TargetArea) -> Option<isize> {
    let x_range = min(0, target.x.0)..max(0, target.x.1) + 1;
    let y_range = target.y.0..10 * max(target.y.0.abs(), target.y.1.abs()); // FIXME : WTF Shanon law :-p
    let mut highest = None;
    for x in x_range.clone() {
        for y in y_range.clone() {
            if let Some(high) = get_highest_position_from_trajectory((x, y), target) {
                highest = Some(max(highest.unwrap_or(high), high));
            }
        }
    }
    highest
}

pub fn display_trajectory() {
    let input = include_str!("../resources/day17_targetarea.txt");
    let area = input.parse().unwrap();
    println!(
        "highest y position reachable : {}",
        find_highest_position(&area).unwrap()
    );
    println!(
        "number of valid trajectories : {}",
        count_valid_trajectories(&area)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let input = "target area: x=20..30, y=-10..-5";

        let t_area: TargetArea = input.parse().unwrap();

        assert_eq!(
            Some(45),
            get_highest_position_from_trajectory((6, 9), &t_area)
        );
        assert_eq!(
            None,
            get_highest_position_from_trajectory((17, -4), &t_area)
        );

        assert_eq!(Some(45), find_highest_position(&t_area));

        assert_eq!(112, count_valid_trajectories(&t_area));
    }
}
