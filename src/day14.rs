use itertools::Itertools;

struct Polymer {
    // store the number of _sequence for x+256 y
    pair_counts: Vec<usize>,
    chemistry: Vec<Option<u8>>,
    // should be handy to avoid counting it once more
    last_component: u8,
}

impl Polymer {
    fn get_index(c1: u8, c2: u8) -> usize {
        (c1 as usize) + 256 * (c2 as usize)
    }

    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let bytes: Vec<_> = lines.next().unwrap().bytes().collect();
        let mut pair_counts = vec![0usize; 256 * 256];
        for (c1, c2) in bytes.iter().tuple_windows() {
            pair_counts[Self::get_index(*c1, *c2)] += 1;
        }

        let last_component = bytes[bytes.len() - 1];

        let mut chemistry = vec![None; 256 * 256];
        for ((c1, c2), new) in lines.filter_map(|l| {
            // let's try without collect_tuple
            let mut vals = l.split(" -> ");
            let mut pair = vals.next()?.bytes();
            let new = vals.next()?.bytes().next()?;
            Some(((pair.next()?, pair.next()?), new))
        }) {
            chemistry[Self::get_index(c1, c2)] = Some(new);
        }

        Self {
            pair_counts,
            chemistry,
            last_component,
        }
    }

    pub fn grow(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut new_assoc = vec![0usize; 256 * 256];
            for (i, count) in self.pair_counts.iter_mut().enumerate() {
                if *count > 0 {
                    if let Some(new_c) = self.chemistry[i] {
                        let c1 = (i % 256) as u8;
                        let c2 = (i >> 8) as u8;
                        new_assoc[Self::get_index(c1, new_c)] += *count;
                        new_assoc[Self::get_index(new_c, c2)] += *count;
                        *count = 0; // these pairs have been split
                    }
                }
            }

            for (j, count) in self.pair_counts.iter_mut().enumerate() {
                if new_assoc[j] != 0 {
                    *count += new_assoc[j];
                }
            }
        }
    }

    pub fn decompose_and_sort_quantities(&self) -> Vec<usize> {
        let mut quantities = [0usize; 256];

        for (i, count) in self.pair_counts.iter().enumerate() {
            if *count > 0 {
                quantities[i % 256] += *count;
            }
        }
        quantities[self.last_component as usize] += 1;

        quantities
            .into_iter()
            .filter(|v| *v > 0)
            .sorted()
            .rev()
            .collect()
    }
}

pub fn display_polymer() {
    let input = include_str!("../resources/day14_chemistry.txt");

    let mut polymer = Polymer::parse(input);
    for i in 1..5 {
        polymer.grow(10);
        let elt_counts = polymer.decompose_and_sort_quantities();
        println!(
            "Difference between most common an least common element after {} steps : {}",
            10 * i,
            elt_counts[0] - elt_counts[elt_counts.len() - 1]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc_example_works() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut polymer = Polymer::parse(input);
        let elt_counts = polymer.decompose_and_sort_quantities();
        assert_eq!(1, elt_counts[0] - elt_counts[elt_counts.len() - 1]);
        polymer.grow(10);
        let elt_counts = polymer.decompose_and_sort_quantities();
        assert_eq!(1588, elt_counts[0] - elt_counts[elt_counts.len() - 1]);
        polymer.grow(30);
        let elt_counts = polymer.decompose_and_sort_quantities();
        assert_eq!(
            2188189693529,
            elt_counts[0] - elt_counts[elt_counts.len() - 1]
        )
    }
}
