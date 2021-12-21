use crate::day9::{get_neighbours_pos_horz_vert, Point};
use itertools::Itertools;

fn get_lowest_risk(input: &str, map_factor: usize) -> usize {
    let base_risks: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split("")
                .filter_map(|c| c.parse::<usize>().ok())
                .collect()
        })
        .collect();

    let dim = (base_risks.len(), base_risks[0].len());
    let mut risks: Vec<Vec<usize>> = vec![vec![0; dim.0 * map_factor]; dim.1 * map_factor];
    for mx in 0..map_factor {
        for my in 0..map_factor {
            for i in 0..dim.0 {
                for j in 0..dim.1 {
                    risks[i + mx * dim.0][j + my * dim.1] =
                        (base_risks[i][j] - 1 + mx + my) % 9 + 1;
                }
            }
        }
    }
    let dim = (dim.0 * map_factor, dim.1 * map_factor);

    let mut best_risks: Vec<Vec<Option<usize>>> = vec![vec![None; dim.0]; dim.1];

    best_risks[0][0] = Some(0);

    let mut current_points: Vec<_> = vec![(0, 0)];
    loop {
        let new_risked_points: Vec<(Point, usize)> = current_points
            .into_iter()
            .map(|p| (p, best_risks[p.0][p.1].unwrap()))
            .flat_map(|(cur, current_risk)| {
                get_neighbours_pos_horz_vert(&cur, &dim)
                    .into_iter()
                    .map(|p| (p, current_risk + risks[p.0][p.1]))
                    .collect_vec()
                    .into_iter()
            })
            .collect();

        for (p, r) in &new_risked_points {
            if best_risks[p.0][p.1].unwrap_or(usize::MAX) > *r {
                best_risks[p.0][p.1] = Some(*r);
            }
        }
        current_points = new_risked_points
            .into_iter()
            .filter_map(|(p, r)| match r {
                r if r == best_risks[p.0][p.1].unwrap() => Some(p),
                _ => None,
            })
            .unique()
            .collect();

        if current_points.is_empty() {
            break;
        }
    }

    best_risks[dim.0 - 1][dim.1 - 1].unwrap()
}

pub fn display_safest_path() {
    let input = include_str!("../ressources/day15_risks.txt");
    println!("lowest_risk {}", get_lowest_risk(input, 1));
    println!(
        "lowest_risk for {}expansion : {}",
        5,
        get_lowest_risk(input, 5)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(40, get_lowest_risk(input, 1));

        assert_eq!(315, get_lowest_risk(input, 5));
    }
}
