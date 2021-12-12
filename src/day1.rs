use itertools::Itertools;

fn parse_depths(report: &'static str) -> impl Iterator<Item = usize> {
    report
        .lines()
        .map(|str_depth| str_depth.parse::<usize>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
}

fn count_depth_incrs(report: &'static str) -> usize {
    parse_depths(report)
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count()
}

fn count_summed_depth_incrs(report: &'static str) -> usize {
    parse_depths(report)
        .tuple_windows()
        .map(|(d1, d2, d3)| d1 + d2 + d3)
        .tuple_windows()
        .filter(|(sum_d1, sum_d2)| sum_d2 > sum_d1)
        .count()
}

pub fn print_depth_incrs() {
    let report = include_str!("day1_sonar_depths.txt");
    println!("number of depth increases : {}", count_depth_incrs(report));
    println!(
        "number of summed depth increases : {}",
        count_summed_depth_incrs(report)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_works() {
        let report = "199
200
208
210
200
207
240
269
260
263
";
        assert_eq!(7, count_depth_incrs(report));
        assert_eq!(5, count_summed_depth_incrs(report));
    }
}
