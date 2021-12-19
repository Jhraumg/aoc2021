use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::once;
use std::mem::take;

struct Polymer {
    sequence: Vec<u8>,
    chemistry: Vec<Option<u8>>,
}

impl Polymer {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let sequence: Vec<_> = lines.next().unwrap().bytes().collect();
        let mut chemistry= vec![None;256*256];
        for ((c1,c2),new) in lines.filter_map(|l| {
                // let's try without collect_tuple
                let mut vals = l.split(" -> ");
                let mut pair = vals.next()?.bytes();
                let mut new = vals.next()?.bytes().next()?;
                Some(((pair.next()?, pair.next()?), new))
            }){
            chemistry[c1 as usize+256*c2 as usize]=Some(new);
        }

        Self {
            sequence,
            chemistry,
        }
    }

    fn get_new_component(&self, c1 :u8, c2:u8) -> Option<u8> {
        self.chemistry[c1 as usize+256*c2 as usize]
    }

    pub fn grow(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut new :Vec<u8> = vec![' ' as u8;2*self.sequence.len()];
            let mut c1 =self.sequence[0];
            let mut chars = self.sequence.iter();
            chars.next(); // TODO : avoid
            for (i,c) in chars.enumerate() {
                new[2*i]=c1;
                new[2*i+1]=self.get_new_component(c1,*c).unwrap_or(' ' as u8);
                c1=*c;
            }
            new[2*self.sequence.len()-1]=self.sequence[self.sequence.len()-1];

            self.sequence = new.into_iter().filter(|c|*c != ' ' as u8).collect();
        }
    }


    pub fn decompose_and_sort_quantities(&self) -> Vec<usize> {
        let mut quantities= [0usize;256];

        for c in &self.sequence {
            quantities[*c as usize]+=1;
        }

        quantities
            .into_iter()
            .filter(|v| *v > 0)
            .sorted()
            .rev()
            .collect()
    }
}

pub fn display_polymer() {
    let input = include_str!("../ressources/day14_chemistry.txt");

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
        println!("polymer : {} elts", polymer.sequence.len());
        polymer.grow(10);
        println!("polymer : {} elts", polymer.sequence.len());
        let elt_counts = polymer.decompose_and_sort_quantities();
        assert_eq!(1588, elt_counts[0] - elt_counts[elt_counts.len() - 1]);
        // polymer.grow(30);
        // let elt_counts = polymer.decompose_and_sort_quantities();
        // assert_eq!(2188189693529,elt_counts[0] - elt_counts[elt_counts.len()-1])
    }
}
