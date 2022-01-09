use crate::day24::Operation::{Add, Div, Equal, Input, Mod, Mul};
use crate::day24::Value::{Reg, Val};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Value {
    Reg(u8),
    Val(isize),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "w" => Ok(Reg(0)),
            "x" => Ok(Reg(1)),
            "y" => Ok(Reg(2)),
            "z" => Ok(Reg(3)),
            val => val.parse::<isize>().map(Val).map_err(|e| anyhow!(e)),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Operation {
    Input(usize),
    Add((usize, Value)),
    Mul((usize, Value)),
    Div((usize, Value)),
    Mod((usize, Value)),
    Equal((usize, Value)),
}

impl Operation {
    fn decode_dual_params(vals: &str) -> Result<(usize, Value), Error> {
        let (a, b) = vals
            .split(' ')
            .collect_tuple()
            .ok_or(anyhow!("add need 2 parameters"))?;
        let a: Value = a.parse()?;
        let b: Value = b.parse()?;
        match a {
            Reg(reg) => Ok((reg as usize, b)),
            Val(_) => Err(anyhow!("first parameter is supposed to be a reg")),
        }
    }
}
impl FromStr for Operation {
    type Err = Error;

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        match &instr[..4] {
            "inp " => instr[4..]
                .parse::<Value>()
                .and_then(|v| {
                    if let Reg(reg) = v {
                        Ok(Input(reg as usize))
                    } else {
                        Err(anyhow!("expecting a register"))
                    }
                })
                .map_err(|e| anyhow!(e)),
            "add " => Self::decode_dual_params(&instr[4..]).map(|(a, b)| Add((a, b))),
            "mul " => Self::decode_dual_params(&instr[4..]).map(|(a, b)| Mul((a, b))),
            "div " => Self::decode_dual_params(&instr[4..]).map(|(a, b)| Div((a, b))),
            "mod " => Self::decode_dual_params(&instr[4..]).map(|(a, b)| Mod((a, b))),
            "eql " => Self::decode_dual_params(&instr[4..]).map(|(a, b)| Equal((a, b))),

            _ => Err(anyhow!("could not translate {}", &instr[..4])),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct ALU<'alu> {
    registers: [isize; 4],

    instructions: &'alu [Operation],
    sp: usize,
    next_val: Option<u8>,
}

impl<'alu> Display for ALU<'alu> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("** ALU\n")?;
        f.write_fmt(format_args!(
            " {}/{} instructions\n",
            self.sp,
            self.instructions.len()
        ))?;
        f.write_fmt(format_args!("sp : {}\n\n", self.sp))?;
        f.write_fmt(format_args!("next : {:?}\n", self.next_val))?;
        f.write_fmt(format_args!("registers :\n{:?}\n", self.registers))
    }
}

impl<'alu> ALU<'alu> {
    fn new(instructions: &'alu [Operation]) -> Self {
        Self {
            registers: [0; 4],
            instructions,
            next_val: None,
            sp: 0,
        }
    }
    fn get(&self, v: &Value) -> isize {
        match v {
            Reg(u) => self.registers[*u as usize],
            Val(v) => *v,
        }
    }

    fn get_mut(&mut self, reg: usize) -> &mut isize {
        &mut self.registers[reg]
    }

    fn process_instruction(&mut self, instr: &Operation) {
        match instr {
            Input(reg) => self.input(*reg),
            Add((a, b)) => self.add(*a, b),
            Mul((a, b)) => self.mul(*a, b),
            Div((a, b)) => self.div(*a, b),
            Mod((a, b)) => self.modulo(*a, b),
            Equal((a, b)) => self.equal(*a, b),
        }
    }

    fn input(&mut self, reg: usize) {
        let val = self.next_val.take();
        let reg = self.get_mut(reg);

        *reg = val.expect("cannot process input instruction without data !") as isize;
        // eprintln!("loading {} into {} ==>\n{}", &val, a, self);
    }

    fn add(&mut self, a: usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("add {} to {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a += reg_b;
    }
    fn mul(&mut self, a: usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("mul {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a *= reg_b;
    }

    fn div(&mut self, a: usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("div {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a /= reg_b;
    }

    fn modulo(&mut self, a: usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("mod {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a %= reg_b;
    }

    fn equal(&mut self, a: usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("eq {} with {}({})", a, reg_b, b.trim());

        let reg_a = self.get_mut(a);
        *reg_a = if *reg_a == reg_b { 1 } else { 0 };
    }

    fn resume(&mut self, next_val: Option<u8>) {
        self.next_val = next_val;
        let sp = self.sp;
        for instruction in &self.instructions[sp..] {
            if let Input(_) = instruction {
                if self.next_val.is_none() {
                    // cannot handle input
                    return;
                }
            }
            self.process_instruction(instruction);
            self.sp += 1;
            // eprintln!("{}\t{:?}",self.sp,self.registers);
        }
    }
}

/// explore the solutions by step
/// for each step, equivalent state is associated with its (min,max) data tuple
/// conflating states at each step is not efficient enough, hence the one level Rayon only
#[allow(dead_code)]
fn get_largest_model_number_accepted_by_monad_breath_first(
    monad: &str,
    size: usize,
) -> (usize, usize) {
    let instructions = decode(monad);

    // each intermediate value is identified by
    // * the registers values
    // * the SP

    let results: Vec<_> = (1..10)
        .into_par_iter()
        .flat_map(|i| {
            let mut alu = ALU::new(&instructions);
            alu.resume(Some(i));
            let mut intermediate_results = vec![(alu, (i as usize, i as usize))];

            for _ in 1..size {
                let new_intermediate_results = intermediate_results
                    .iter()
                    .flat_map(|(alu, (min, max))| {
                        (1..10).map(move |i| {
                            let mut new_alu = *alu;
                            new_alu.resume(Some(i));
                            (new_alu, (10 * min + i as usize, 10 * max + i as usize))
                        })
                    })
                    .fold(
                        HashMap::<(usize, [isize; 4]), (ALU, (usize, usize))>::new(),
                        |mut acc, (v, (loc_min, loc_max))| {
                            let key = (v.sp, v.registers);
                            let (new_min, new_max) =
                                if let Some((_alu, (cur_min, cur_max))) = acc.get(&key) {
                                    (min(loc_min, *cur_min), max(loc_max, *cur_max))
                                } else {
                                    (loc_min, loc_max)
                                };
                            acc.insert(key, (v, (new_min, new_max)));

                            acc
                        },
                    );

                intermediate_results = new_intermediate_results.into_values().collect();
            }

            intermediate_results
                .into_par_iter()
                .filter(|(alu, _)| alu.registers[3] == 0)
        })
        .collect();

    let (fmin, fmax) = results
        .into_iter()
        .fold((usize::MAX, 0), |(fmin, fmax), (_, (lmin, lmax))| {
            (min(fmin, lmin), max(fmax, lmax))
        });

    eprintln!("max checked : {} ", fmax);
    eprintln!("min checked : {} ", fmin);
    (fmin, fmax)
}

fn decode(program: &str) -> Vec<Operation> {
    program
        .lines()
        .filter_map(|l| {
            if let Ok(oper) = l.parse() {
                Some(oper)
            } else {
                eprintln!("could not parse '{}'", l);
                None
            }
        })
        .collect()
}

/// explore the solution from max to min
/// stack based
/// already failed explored situation (from a larger serial) are not explored twice
/// TODO : start by both ends, but store guaranteed_ko during the journey in the same container
/// current version just split the job by first digit
/// failed state are not shared this way (should spawn real task to do so, and then merge the fails at each return)
fn get_largest_model_number_accepted_by_monad_depth_first(
    monad: &str,
    size: usize,
) -> (usize, usize) {
    let instructions = decode(monad);

    let checked_max = vec![9u8; size];
    let checked_min = vec![1u8; size];

    let results: Vec<(usize, usize)> = (1..10)
        .into_par_iter()
        .filter_map(|i| {
            let mut data = checked_max.clone();
            data[0] = i;

            let mut max_checked: Option<usize> = None;
            let mut min_checked: Option<usize> = None;
            let mut last_checked: Option<Vec<u8>> = None;

            let mut intermediate_results: VecDeque<ALU> = VecDeque::with_capacity(size);

            // stores partial results which always end up KO
            // any other similar result at the same state cannot succeed
            let mut guaranteed_ko_partial_states: HashSet<(usize, [isize; 4])> =
                HashSet::with_capacity(3usize.pow(size as u32));

            intermediate_results.push_back(ALU::new(&instructions));
            loop {
                while intermediate_results.len() <= size {
                    // since the first value is BEFORE the first data
                    let base = intermediate_results
                        .get(intermediate_results.len() - 1)
                        .expect("intermediate_result should not be empty");

                    let mut alu = *base;
                    alu.resume(Some(data[intermediate_results.len() - 1]));

                    if intermediate_results.len() < size {
                        if guaranteed_ko_partial_states.contains(&(alu.sp, alu.registers)) {
                            // eprintln!("skipping {:?} {}, seen with {:?}",&data, intermediate_results.len()-1, garantied_ko_partial_states.get(&(alu.sp,alu.registers)));
                            data[intermediate_results.len()..]
                                .copy_from_slice(&checked_min[intermediate_results.len()..]);

                            break;
                        }
                    } else {
                        // monad check
                        if alu.registers[3] == 0 {
                            let checked = data.iter().fold(0, |acc, v| acc * 10 + (*v as usize));
                            if max_checked.is_none() {
                                max_checked = Some(checked);
                            }
                            min_checked = Some(checked);
                            last_checked = Some(data.clone());
                        }
                    }
                    intermediate_results.push_back(alu);
                }

                if let Some(i) = (1..size).rev().find(|j| data[*j] > 1) {
                    let necessary_pop = intermediate_results.len();
                    if necessary_pop > i {
                        let mut last_pop = None;
                        for _ in i + 1..necessary_pop {
                            if let Some(alu) = intermediate_results.pop_back() {
                                last_pop = Some(alu);
                            }
                        }
                        if let Some(alu) = last_pop {
                            // that's the last to be removed, next is 1 level upper
                            // We can mark is as guaranteed KO **only** if none of its children succeed
                            // thus if the last registered is not one of its children
                            if i + 1 < size
                                && !last_checked
                                    .as_deref()
                                    .map(|ok_data| ok_data[..i + 1] == data[..i + 1])
                                    .unwrap_or(false)
                            {
                                guaranteed_ko_partial_states.insert((alu.sp, alu.registers));
                            };
                        }
                    }
                    data[i] -= 1;

                    data[i + 1..].copy_from_slice(&checked_max[i + 1..]);
                } else {
                    break;
                }
            }

            min_checked.map(|lmin| {
                (
                    lmin,
                    max_checked.expect("cannot have minc_checked without max_checked"),
                )
            })
        })
        .collect();

    results
        .into_iter()
        .fold((usize::MAX, 0), |(fmin, fmax), (lmin, lmax)| {
            (min(fmin, lmin), max(fmax, lmax))
        })
}

pub fn print_larget_serial_accepted_by_monad() {
    let program = include_str!("../resources/day24_monad.txt");
    let (min_checked, max_checked) =
        get_largest_model_number_accepted_by_monad_depth_first(program, 14);
    println!("largest serial accepted {}", max_checked);
    println!("lowest serial accepted {}", min_checked);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operationns_can_be_decoded() {
        let program = "\
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
";
        for instruction in program.lines() {
            let _oper: Operation = instruction.parse().expect("parsing Operation");
        }
    }

    #[test]
    fn monad_works_for_small_serial() {
        let program = include_str!("../resources/day24_monad.txt");
        println!(
            "largest serial accepted {}",
            get_largest_model_number_accepted_by_monad_depth_first(program, 7).0
        );
    }

    #[test]
    fn monad_can_be_checked() {
        let program = include_str!("../resources/day24_monad.txt");

        let instructions = decode(program);

        let data = vec![1, 1, 9, 9];
        let mut alu = ALU::new(&instructions);
        for input in data {
            alu.resume(Some(input));
            println!("***\n{},", alu);
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn monad_can_be_checked_breath_first() {
        let program = include_str!("../resources/day24_monad.txt");
        assert_eq!(
            (91811211611981, 92928914999991),
            get_largest_model_number_accepted_by_monad_breath_first(program, 14)
        );
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn monad_can_be_checked_depth_first() {
        let program = include_str!("../resources/day24_monad.txt");
        assert_eq!(
            (91811211611981, 92928914999991),
            get_largest_model_number_accepted_by_monad_depth_first(program, 14)
        );
    }
}
