use super::day9::*;
use std::collections::HashSet;

fn parse_energy_levels(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split("")
                .filter_map(|c| c.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn count_number_of_flashes_for_step(energies: &mut [Vec<usize>]) -> (usize, bool) {
    let dim = (energies.len(), energies.get(0).unwrap().len());
    let mut flashed: HashSet<Point> = HashSet::with_capacity(dim.0 * dim.1);

    energies.iter_mut()
        .flat_map(|l|l.iter_mut())
        .for_each(|e|*e+=1);

    loop {
        let next_flashes: Vec<Point> = energies
            .iter()
            .enumerate()
            .flat_map(move |(x, l)| l.iter().enumerate().map(move |(y, e)| (x, y, *e)))
            .filter(|(_, _, e)| *e > 9)
            .map(|(x, y, _)| (x, y))
            .filter(|p| !flashed.contains(p))
            .collect();

        if next_flashes.is_empty() {
            break;
        } else {
            for (x, y) in &next_flashes {
                flashed.insert((*x, *y));
            }
            for (x, y) in next_flashes
                .into_iter()
                .flat_map(|p| get_neighbours_pos_diag(&p, &dim).into_iter())
            {
                energies[x][y] += 1;
            }
        }
    }
    for (x, y) in &flashed {
        energies[*x][*y] = 0;
    }
    let flashes_count = flashed.len();
    let all_flashed = flashes_count == dim.0 * dim.1;
    (flashes_count, all_flashed)
}
fn sum_flashes(input: &str, steps: usize) -> usize {
    let mut energies = parse_energy_levels(input);
    let mut flashes_count = 0;
    for _ in 0..steps {
        flashes_count += count_number_of_flashes_for_step(&mut energies).0;
    }
    flashes_count
}
fn get_first_all_flashed_step(input: &str) -> usize {
    let mut first = 1;
    let mut energies = parse_energy_levels(input);
    while !count_number_of_flashes_for_step(&mut energies).1 {
        first += 1;
    }
    first
}

pub fn display_octopuses_flash_count() {
    let input = include_str!("../ressources/day11_octopuses_energy.txt");
    println!(
        "number of flashes after 100 steps {}",
        sum_flashes(input, 100)
    );
    println!(
        "first step during which all octopuses flash {}",
        get_first_all_flashed_step(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_simple_example_works() {
        let input = "11111
19991
19191
19991
11111";
        assert_eq!(9, sum_flashes(input, 1));
    }
    #[test]
    fn aoc_example_works() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(35, sum_flashes(input, 2));
        assert_eq!(1656, sum_flashes(input, 100));

        assert_eq!(195, get_first_all_flashed_step(input));
    }
}
