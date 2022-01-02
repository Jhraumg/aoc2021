use std::fmt::Formatter;
use std::fmt::Display;
use std::str::FromStr;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::day24::Operation::{Add, Div, Input, Mod, Mul,Equal};
use crate::day24::Value::{Reg, Val};

#[derive(Debug,Clone,Copy)]
enum Value {
    Reg(usize),
    Val(isize),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim(){
            "w" => Ok(Reg(0)),
            "x" => Ok(Reg(1)),
            "y" => Ok(Reg(2)),
            "z" => Ok(Reg(3)),
            val => val.parse::<isize>().and_then(|v|Ok(Val(v))).or_else(|a|Err(anyhow!(a))),
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum Operation {
    Input(usize),
    Add((usize,Value)),
    Mul((usize,Value)),
    Div((usize,Value)),
    Mod((usize,Value)),
    Equal((usize, Value)),
}

impl Operation {
    fn decode_dual_params(vals : &str)-> Result<(usize, Value),Error> {
        let (a ,b )=vals.split(" ").collect_tuple().ok_or(anyhow!("add need 2 parameters"))?;
        let a:Value = a.parse()?;
        let b:Value=b.parse()?;
        match a {
            Reg(reg) => Ok((reg,b)),
            Val(_) => Err(anyhow!("first parameter is supposed to be a reg"))
        }
    }
}
impl FromStr for Operation {
    type Err = Error;

    fn from_str(instr :&str) -> Result<Self, Self::Err> {
        match &instr[..4] {
            "inp " => instr[4..].parse::<Value>().and_then(|v| if let Reg(reg)= v {Ok(Input(reg))}else{Err(anyhow!("expecting a register"))}).or_else(|e| Err(anyhow!(e))),
            "add " => Self::decode_dual_params(&instr[4..]).and_then(|(a, b)| Ok(Add((a, b)))),
            "mul " => Self::decode_dual_params(&instr[4..]).and_then(|(a, b)| Ok(Mul((a, b)))),
            "div " => Self::decode_dual_params(&instr[4..]).and_then(|(a, b)| Ok(Div((a, b)))),
            "mod " => Self::decode_dual_params(&instr[4..]).and_then(|(a, b)| Ok(Mod((a, b)))),
            "eql " => Self::decode_dual_params(&instr[4..]).and_then(|(a, b)| Ok(Equal((a, b)))),

            _ => Err(anyhow!("could not translate {}",&instr[..4]))
        }
    }
}

struct ALU<'alu> {

    registers : [isize;4],

    instructions: &'alu[Operation],
    sp : usize,
    data: Vec<u8>,
    dp: usize,
}

impl<'alu> Display for ALU<'alu>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("** ALU\n")?;
        f.write_fmt(format_args!(" {}/{} instructions\n", self.sp,self.instructions.len()))?;
        f.write_fmt(format_args!("data : {}\n", self.data.iter().join(",")))?;
        f.write_fmt(format_args!("sp : {}\n\n", self.dp))?;
        f.write_fmt(format_args!("registers :\n{:?}\n", self.registers))
    }
}

impl<'alu>  ALU<'alu>  {
    fn new()->Self{
        Self{
            registers : [0;4],
            instructions: &[],
            data: vec![],
            dp: 0,
            sp:0,
        }
    }
    fn get(&self, v: &Value) -> isize {
        match v {
            Reg(u) => self.registers[*u],
            Val(v) => *v,
        }
    }

    fn get_mut(&mut self, reg : usize) -> &mut isize {
        &mut self.registers[reg]
    }

    fn process_instruction(&mut self, instr : &Operation){
        match instr {
            Input(reg) => self.input(*reg),
            Add((a,b)) => self.add(*a,b),
            Mul((a,b)) =>self.mul(*a,b),
            Div((a,b)) => self.div(*a,b),
            Mod((a,b)) => self.modulo(*a,b),
            Equal((a,b)) => self.equal(*a,b),
        }
    }

    fn input(&mut self, reg :usize) {
        let val = self.data[self.dp];
        let reg = self.get_mut(reg);

        *reg=val as isize;
        self.dp +=1;
        // eprintln!("loading {} into {} ==>\n{}", &val, a, self);
    }

    fn add(&mut self, a:usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("add {} to {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a += reg_b;
    }
    fn mul(&mut self, a:usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("mul {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a *= reg_b;
    }

    fn div(&mut self, a:usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("div {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a /= reg_b;
    }

    fn modulo(&mut self, a:usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("mod {} by {}({})", a, reg_b, b.trim());
        let reg_a = self.get_mut(a);
        *reg_a %= reg_b;
    }

    fn equal(&mut self, a:usize, b: &Value) {
        let reg_b = self.get(b);
        // eprintln!("eq {} with {}({})", a, reg_b, b.trim());

        let reg_a = self.get_mut(a);
        *reg_a = if *reg_a==reg_b {1} else {0};
    }

    fn run(&mut self, program : &'alu [Operation], data :Vec<u8>){
        // reinit
        self.registers=[0;4];
        self.dp =0;
        self.sp=0;
        self.instructions = program;
        self.data = data;


        while self.sp < self.instructions.len() {
            let instruction = self.instructions[self.sp];
            self.process_instruction(&instruction);
            self.sp+=1;
        }
    }

}

fn monad_check(program : &Vec<Operation>, serial : Vec<u8>) -> bool {

    let mut alu = ALU::new();
    alu.run(&program, serial);

    if alu.registers[3] == 0 {
        println!("checked OK :\n{}",alu);
    }
    alu.registers[3] == 0
}

fn get_largest_model_number_accepted_by_monad(monad : &str, size:usize) -> usize {
    let instructions : Vec<Operation> = monad.lines().filter_map(|l| {
        if let Ok(oper) = l.parse(){
            Some(oper)
        }else{
            eprintln!("could not parse '{}'",l);
            None
        }
    }).collect();

    let mut data = vec![9u8;size];
    loop {
        // if monad_check(&instructions, data.clone()) {
        //     return data.iter().fold(0,|acc,v|acc*10+  *v as usize)
        // }
        if let Some(i) = (0..size).rev().find(|j| data[*j]> 1){
            data[i]-=1;
            for j in i+1..size{
                data[j]=9;
            }
            // if i < 5 {
            //     eprintln!("next : {:?}",data);
            // }

        }else{
            break
        }
    }

    0
}

pub fn print_larget_serial_accepted_by_monad(){
    let program = include_str!("../ressources/day24_monad.txt");
    println!("largest serial accepted {}", get_largest_model_number_accepted_by_monad(program,14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operationns_can_be_decoded(){

        let program="\
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
        for instruction in program.lines(){
            let oper : Operation=instruction.parse().expect("parsing Operation");
        }
    }

    #[test]
    fn monad_works_for_small_serial(){
        let program = include_str!("../ressources/day24_monad.txt");
        println!("largest serial accepted {}", get_largest_model_number_accepted_by_monad(program,10));
    }
}
