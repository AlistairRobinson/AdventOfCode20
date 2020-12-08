use crate::{Fail, Input, Solution};
use std::cmp::max;
use std::collections::HashSet;

pub static DATA_PATH: &str = "data/day8.txt";
pub static TEST_PATH: &str = "data/test/day8.txt";
pub static TEST_VALUES: (&str, &str) = ("5", "8");

#[derive(Debug, Clone)]
pub enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

#[derive(Debug)]
pub struct Program {
    pc: usize,
    acc: i32,
    instr: Vec<Instruction>,
    halted: bool,
}

#[derive(Debug)]
pub struct State {
    pc: usize,
    acc: i32,
    halted: bool,
}

impl From<&mut Program> for State {
    fn from(p: &mut Program) -> State {
        State {
            pc: p.pc,
            acc: p.acc,
            halted: p.halted,
        }
    }
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
            halted: false,
        }
    }
}

impl Iterator for Program {
    type Item = State;
    fn next(&mut self) -> Option<State> {
        if self.halted {
            None
        } else if self.pc >= self.instr.len() {
            self.halted = true;
            Some(self.into())
        } else {
            match self.instr[self.pc] {
                Instruction::ACC(i) => self.acc += i,
                Instruction::JMP(i) => self.pc = max(self.pc as i32 + i - 1, 0) as usize,
                Instruction::NOP(_) => self.acc += 0,
            }
            self.pc = (self.pc + 1) as usize;
            Some(self.into())
        }
    }
}

impl Program {
    fn new(instr: Vec<Instruction>) -> Program {
        Program {
            pc: 0,
            acc: 0,
            instr: instr,
            halted: false,
        }
    }

    fn run(&mut self) -> State {
        let mut ins: HashSet<usize> = HashSet::new();
        self.into_iter()
            .skip_while(|p| ins.insert(p.pc) && !p.halted)
            .next()
            .unwrap()
    }

    fn halts(&mut self) -> Option<State> {
        let result: State = self.run();
        match result.halted {
            true => Some(result),
            false => None,
        }
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let mut input: Program = self.into();
        Ok(input.run().acc.to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Program = self.into();
        Ok(input
            .instr
            .iter()
            .enumerate()
            .filter_map(swappable)
            .fold(None, |b, i| match b {
                Some(_) => b,
                None => Program::new(swap(input.instr.clone(), i)).halts(),
            })
            .unwrap()
            .acc
            .to_string())
    }
}

fn swappable(t: (usize, &Instruction)) -> Option<usize> {
    match t.1 {
        Instruction::ACC(_) => None,
        Instruction::JMP(_) => Some(t.0),
        Instruction::NOP(_) => Some(t.0),
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
