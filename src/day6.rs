use itertools::Itertools;

#[derive(Debug)]
struct Lanternfishes {
    count_by_ages : [usize;9],
}

impl Lanternfishes {
    pub fn new(fishes : &Vec<usize>) -> Lanternfishes {
        println!("{:?} ... {:?}", &fishes[..3], &fishes[fishes.len()-3..]);
        let mut count_by_ages = [0usize;9];
        for fish in fishes {
            count_by_ages[*fish]+=1;
        }
        Lanternfishes{count_by_ages}
    }
    pub fn parse(fishes : &str) -> Lanternfishes {

        Self::new(&fishes.trim().split(",").filter_map(|v|v.parse::<usize>().ok()).collect_vec())
    }
    pub fn count(&self) -> usize {
        self.count_by_ages.iter().sum()
    }

    // tbh, we could probably directly count after n days
    pub fn grow_1day(&mut self) {
        let mut new_counts= [0usize;9];
        new_counts[8]=self.count_by_ages[0];
        for i in 0..8{
            new_counts[i]=self.count_by_ages[i+1]
        }
        new_counts[6]+=self.count_by_ages[0];
        self.count_by_ages = new_counts;
    }
}

fn count_lanternfishes_after(population :&str, duration : usize) -> usize {
    let mut pop = Lanternfishes::parse(population);
    for d in 0..duration {
        pop.grow_1day();
    }
    pop.count()
}
pub fn print_lanternfishes_counts(){

    let lanternfishes = include_str!("day6_lanternfishes.txt");
    for d in [0,80,256 ] {
        println!("after {} days, there are {} fishes",d,
                 count_lanternfishes_after(lanternfishes,d));

    }
}

#[cfg(test)]

mod tests {
use super::*;
    #[test]
    fn aoc_example_works() {
        let lanternfishes = "3,4,3,1,2";

        assert_eq!(5934, count_lanternfishes_after(lanternfishes, 80));
        assert_eq!(26984457539, count_lanternfishes_after(lanternfishes, 256));
    }
}