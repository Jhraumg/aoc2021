use std::ops::Not;
use itertools::Itertools;

#[derive(Debug)]
struct Diagnosis {
    length : usize,
    gamma : usize,
    epsilon :usize,
    oxygen : usize,
    co2 : usize,
}

impl Diagnosis {
    fn _epsilon(length: usize, gamma:usize) -> usize{
        (! gamma) & ((1<<length) -1)
    }

    pub fn select_diag_from_rating(diag_bits: &Vec<Vec<u8>>, selector : impl Fn(u8,u8)->bool) -> usize {
        let mut candidates :Vec<bool>= vec![true;diag_bits.len()];
        let mut number_of_candidates = diag_bits.len();

        let diag_len = diag_bits[0].len();
        for idx in 0..diag_len {
            let bit_counts = diag_bits.iter().enumerate().filter(|(idx, _)|candidates[*idx])
                .map(|(_,bit)|bit[idx])
                .fold((0,0),|acc, val|match val{
                    0 => (acc.0+1,acc.1),
                    1 => (acc.0,acc.1+1),
                    _ => acc
                });
            let most_common_bit = if bit_counts.0 > bit_counts.1 {0}else{1};
            for cand_idx in 0..diag_bits.len() {
                if candidates[cand_idx] {
                    if ! selector(diag_bits[cand_idx][idx], most_common_bit){
                        candidates[cand_idx]=false;
                        number_of_candidates-=1;
                    }
                }
                if number_of_candidates <2 {
                    break
                }
            }

        }


        let (idx, result)=diag_bits.iter().enumerate().find(|(idx, _)|candidates[*idx]).unwrap();
        // dbg!(idx, result);
        Diagnosis::from_bits_to_val(result)
    }

    pub fn from_bits_to_val(bits : &Vec<u8>) -> usize {
        bits.iter().fold(0usize,|acc, bit| acc * 2 + *bit as usize)
    }

    pub fn new(report : &str) -> Diagnosis {
        let inputs = report.lines().filter(|l| !l.is_empty())
            .map(|l|l.split("").filter_map(|d|d.parse::<u8>().ok() ).collect_vec())
            .collect_vec();

        let length = inputs.iter().map(|l|l.len()).max().unwrap();
        let gamma_bits = (0..length).map(|i|{
            let (zeroes_nb, ones_nb ) = *&inputs.iter()
                .map(|input| input[i])
                .map(|val|match val {
                    0 => (1,0),
                    1 => (0,1),
                    _ => (0,0)
                }).fold((0usize,0usize), |acc, val| (acc.0 + val.0, acc.1 + val.1 ));
             if zeroes_nb> ones_nb { 0u8 } else { 1u8 } // This will ensure than 1 is considered if counts  are equal
        }).collect_vec();
        let gamma : usize = Diagnosis::from_bits_to_val(&gamma_bits);
        let epsilon=Diagnosis::_epsilon(length, gamma);

        let oxygen = Diagnosis::select_diag_from_rating(&inputs,  |val, most| val==most);
        let co2 = Diagnosis::select_diag_from_rating(&inputs,  |val, most| val!=most);


        Diagnosis{length, gamma, epsilon, oxygen, co2}

    }
}

pub fn print_power() {
    let report = include_str!("day3_diagnosis.txt");
    let diag = Diagnosis::new(report);
    println!("diagnosis : {:?}", diag);
    println!("power : {:?}", diag.gamma * diag.epsilon);
    println!("oxygen*co2 : {:?}", diag.oxygen * diag.co2);


}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn aoc_example(){
        let report="
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
// 10110
        assert_eq!(1, Diagnosis::_epsilon(5, 0b11110));

        let full_diag = Diagnosis::new(report);
        assert_eq!(5, full_diag.length);
        assert_eq!(22, full_diag.gamma);
        assert_eq!(9, full_diag.epsilon);
        assert_eq!(198, full_diag.gamma * full_diag.epsilon);

        assert_eq!(23, full_diag.oxygen);
        assert_eq!(10, full_diag.co2);

    }
}