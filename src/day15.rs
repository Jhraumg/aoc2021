use crate::day9::{get_neighbours_pos_horz_vert, Point};
use std::collections::{HashSet};

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

    let mut current_points: HashSet<_> = vec![(0, 0)].into_iter().collect();
    loop {
        let new_risked_points: Vec<(Point, usize)> = current_points
            .iter()
            .flat_map(|cur| {
                let current_risk = best_risks[cur.0][cur.1].unwrap();
                let neighbours: Vec<_> = get_neighbours_pos_horz_vert(cur, &dim)
                    .into_iter()
                    .filter_map(|n| {
                        let new_risk = current_risk + risks[n.0][n.1];
                        let previous_best_risk = best_risks[n.0][n.1].unwrap_or(usize::MAX);

                        if new_risk < previous_best_risk {
                            Some((n, new_risk))
                        } else {
                            None
                        }
                    })
                    .collect(); //TODO : see how to avoid this
                neighbours.into_iter()
            })
            .collect();

        for (p, r) in &new_risked_points {
            if best_risks[p.0][p.1].unwrap_or(usize::MAX) > *r {
                best_risks[p.0][p.1] = Some(*r);
            }
        }
        if new_risked_points.is_empty() {
            break;
        }
        current_points = new_risked_points.into_iter().map(|(p, _)| p).collect();
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
