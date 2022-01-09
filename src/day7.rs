fn compute_alignment_necessary_fuel(pos: &str, fuel_law: impl Fn(usize) -> usize) -> usize {
    let pos: Vec<_> = pos
        .trim()
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect();

    // let be naive
    let min_pos = *pos.iter().min().unwrap();
    let max_pos = *pos.iter().max().unwrap();
    (min_pos..max_pos + 1)
        .map(|target| {
            pos.iter()
                .map(|p| fuel_law(if *p > target { p - target } else { target - p }))
                .sum()
        })
        .min()
        .unwrap()
}

pub fn print_crab_alignment() {
    let crabs_pos = include_str!("../resources/day7_crabs_pos.txt");
    println!(
        "fuel necessary to align  crabs (simple law) {}",
        compute_alignment_necessary_fuel(crabs_pos, |d| d)
    );
    println!(
        "fuel necessary to align  crabs (cumulative law){}",
        compute_alignment_necessary_fuel(crabs_pos, |d| (d * (d + 1)) >> 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc_example_works() {
        let crabs_pos = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, compute_alignment_necessary_fuel(crabs_pos, |d| d));

        // 1 + 2 + ...+ n = n*(n+1)/2  (and /2 ==  >>1)
        assert_eq!(
            168,
            compute_alignment_necessary_fuel(crabs_pos, |d| (d * (d + 1)) >> 1)
        );
    }
}
