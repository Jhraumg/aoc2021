use itertools::Itertools;

fn parse_heights(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|c| c.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn get_dim(map: &[Vec<usize>]) -> Point {
    let max_x = map.len();
    let max_y = map.iter().map(|line| line.len()).max().unwrap();
    (max_x, max_y)
}

pub type Point = (usize, usize);

fn get_neighbours_pos(
    pos: &Point,
    dim: &Point,
    selector: impl Fn(Point, Point) -> bool,
) -> Vec<Point> {
    let (x, y) = pos;
    let (max_x, max_y) = dim;

    let x_range = match x {
        0 => *x..x + 2,
        max if *max == *max_x - 1 => x - 1..x + 1,
        _ => x - 1..x + 2,
    };
    let y_range = match y {
        0 => *y..y + 2,
        max if *max == *max_y - 1 => y - 1..y + 1,
        _ => y - 1..y + 2,
    };
    x_range
        .flat_map(|nx| y_range.clone().map(move |ny| (nx, ny)))
        .filter(|(nx, ny)| selector((*x, *y), (*nx, *ny)))
        .collect()
}

// thats'not what we're asked :-p
pub fn get_neighbours_pos_diag(pos: &Point, dim: &Point) -> Vec<Point> {
    get_neighbours_pos(pos, dim, |(x, y), (nx, ny)| nx != x || ny != y)
}

fn get_neighbours_pos_horz_vert(pos: &Point, dim: &Point) -> Vec<Point> {
    get_neighbours_pos(pos, dim, |(x, y), (nx, ny)| (nx != x) ^ (ny != y))
}

fn get_low_points(heights: &[Vec<usize>]) -> Vec<Point> {
    let dim = get_dim(heights);

    let height_points = heights
        .iter()
        .enumerate()
        .flat_map(|(x, c)| c.iter().enumerate().map(move |(y, height)| (x, y, *height)));

    height_points
        .filter(|(x, y, h)| {
            let neighbours = get_neighbours_pos_horz_vert(&(*x, *y), &dim);
            *h < neighbours
                .iter()
                .map(|(x, y)| heights[*x][*y])
                .min()
                .unwrap()
        })
        .map(|(x, y, _)| (x, y))
        .collect()
}

fn sum_low_point_risks(heights: &[Vec<usize>]) -> usize {
    let low_points = get_low_points(heights);
    low_points.into_iter().map(|(x, y)| heights[x][y] + 1).sum()
}

fn get_flowing_points(point: &Point, dim: &Point, heights: &[Vec<usize>]) -> Vec<Point> {
    let height = heights[point.0][point.1];
    get_neighbours_pos_horz_vert(point, dim)
        .into_iter()
        .filter(|(x, y)| height < heights[*x][*y] && heights[*x][*y] < 9)
        .collect()
}

fn get_bassin(low_point: &Point, dim: &Point, heights: &[Vec<usize>]) -> Vec<Point> {
    let mut bassin: Vec<Point> = vec![];

    let mut new_points = vec![*low_point];
    while !new_points.is_empty() {
        let mut new_new_points: Vec<_> = new_points
            .iter()
            .flat_map(|(x, y)| get_flowing_points(&(*x, *y), dim, heights).into_iter())
            .filter(|(x, y)| !bassin.contains(&(*x, *y)))
            .unique()
            .collect();
        bassin.append(&mut new_points);
        new_points.append(&mut new_new_points);
    }
    bassin
}

fn multiply_bassins(heights: &[Vec<usize>]) -> usize {
    let dim = get_dim(heights);
    let low_points = get_low_points(heights);
    let bassins_lengths = low_points
        .iter()
        .map(|p| get_bassin(p, &dim, heights))
        .map(|b| b.len())
        .sorted()
        .rev();

    bassins_lengths.take(3).product()
}

pub fn display_smoke_risks() {
    let input = include_str!("../ressources/day9_heights.txt");
    let heights = parse_heights(input);
    let risk = sum_low_point_risks(&heights);

    println!("sum of risk of all low points {}", risk);

    println!(
        "product of 3 larghest bassins {}",
        multiply_bassins(&heights)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678
";
        let heights = parse_heights(input);

        assert_eq!(15, sum_low_point_risks(&heights));
        assert_eq!(1134, multiply_bassins(&heights));
    }
}
