use crate::{Fail, Input, Solution};
use std::collections::HashSet;

pub static DATA_PATH: &str = "data/day8.txt";
pub static TEST_PATH: &str = "data/test/day8.txt";
pub static TEST_VALUES: (&str, &str) = ("5", "5");

#[derive(Clone, Debug)]
pub enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

#[derive(Clone, Debug)]
pub struct Program {
    pc: usize,
    acc: i32,
    instr: Vec<Instruction>,
}

impl From<&Input> for Program {
    fn from(input: &Input) -> Program {
        Program {
            pc: 0,
            acc: 0,
            instr: input
                .data
                .lines()
                .map(|x| x.split(" ").collect::<Vec<&str>>())
                .map(|x| match x[0] {
                    "acc" => Instruction::ACC(x[1].parse().unwrap()),
                    "jmp" => Instruction::JMP(x[1].parse().unwrap()),
                    "nop" => Instruction::NOP(x[1].parse().unwrap()),
                    _ => Instruction::NOP(0),
                })
                .collect(),
        }
    }
}

impl Iterator for Program {
    type Item = Program;
    fn next(&mut self) -> Option<Program> {
        if self.pc >= self.instr.len() {
            return None;
        }
        match self.instr[self.pc] {
            Instruction::ACC(i) => self.acc += i,
            Instruction::JMP(i) => self.pc = ((self.pc as i32 - 1) + i) as usize,
            Instruction::NOP(_) => self.acc += 0,
        }
        self.pc += 1;
        Some(self.clone())
    }
}

impl Program {
    fn halts(&mut self) -> Option<i32> {
        let mut ins: HashSet<usize> = HashSet::new();
        let result: Program = self
            .into_iter()
            .take_while(|p| ins.insert(p.pc))
            .last()
            .unwrap();
        if result.pc < result.instr.len() {
            None
        } else {
            Some(result.acc)
        }
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Program = self.into();
        let mut ins: HashSet<usize> = HashSet::new();
        Ok(input
            .into_iter()
            .take_while(|p| ins.insert(p.pc))
            .last()
            .unwrap()
            .acc
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Program = self.into();
        Ok(input
            .instr
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Instruction::ACC(_) => None,
                Instruction::JMP(_) => Some(i),
                Instruction::NOP(_) => Some(i),
            })
            .fold(None, |b, i| match b {
                Some(_) => b,
                None => (Program {
                    pc: 0,
                    acc: 0,
                    instr: swap(input.instr.clone(), i),
                })
                .halts(),
            })
            .unwrap()
            .to_string())
    }
}

fn swap(mut instr: Vec<Instruction>, index: usize) -> Vec<Instruction> {
    match instr[index] {
        Instruction::JMP(n) => instr[index] = Instruction::NOP(n),
        Instruction::NOP(n) => instr[index] = Instruction::JMP(n),
        _ => (),
    }
    instr
}
