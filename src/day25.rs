use crate::day25::Cucumber::{Down, Right};

enum Cucumber {
    Right,
    Down,
}
type Floor = Vec<Vec<Option<Cucumber>>>;

fn parse_input(input: &str) -> Floor {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '>' => Some(Right),
                    'v' => Some(Down),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn move_right(floor: &mut Floor) -> usize {
    let len = floor[0].len();

    let moves: Vec<_> = floor
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter().enumerate().filter_map(move |(j, c)| {
                let len = l.len();
                match c {
                    Some(Right) => {
                        if l[(j + 1) % len].is_none() {
                            Some((i, j))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
        })
        .collect();
    let mut count = 0;
    for (i, j) in moves {
        count += 1;
        floor[i][(j + 1) % len] = floor[i][j].take();
    }
    count
}
fn move_down(floor: &mut Floor) -> usize {
    let depth = floor.len();

    let the_floor: &Floor = floor;

    let moves: Vec<_> = floor
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter().enumerate().filter_map(move |(j, c)| match c {
                Some(Down) => {
                    if the_floor[(i + 1) % depth][j].is_none() {
                        Some((i, j))
                    } else {
                        None
                    }
                }
                _ => None,
            })
        })
        .collect();
    let mut count = 0;
    for (i, j) in moves {
        count += 1;
        floor[(i + 1) % depth][j] = floor[i][j].take();
    }
    count
}

fn count_steps_before_static(input: &str) -> usize {
    let mut floor = parse_input(input);
    let mut count = 0;

    loop {
        count += 1;

        let mut moved = 0;
        moved += move_right(&mut floor);
        moved += move_down(&mut floor);
        if moved == 0 {
            break;
        }
    }

    count
}

pub fn find_spot_on_sea_floor() {
    let input = include_str!("../resources/day25_sea_floor.txt");
    println!(
        "cucumbers stop moving after {} steps",
        count_steps_before_static(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples_work() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

        assert_eq!(58, count_steps_before_static(input));
    }
}
