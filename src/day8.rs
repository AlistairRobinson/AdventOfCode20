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

#[derive(Debug, Clone)]
pub struct State {
    pc: usize,
    acc: i32,
    halted: bool,
}

impl State {
    fn new() -> State {
        State { pc: 0, acc: 0, halted: false }
    }
}

#[derive(Debug)]
pub struct Program {
    state: State,
    instr: Vec<Instruction>,
}

impl From<&Input> for Program {
    fn from(input: &Input) -> Program {
        Program {
            state: State::new(),
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
    type Item = State;
    fn next(&mut self) -> Option<State> {
        if self.state.halted {
            None
        } else if self.state.pc >= self.instr.len() {
            self.state.halted = true;
            Some(self.state.clone())
        } else {
            match self.instr[self.state.pc] {
                Instruction::ACC(i) => self.state.acc += i,
                Instruction::JMP(i) => self.state.pc = max(self.state.pc as i32 + i - 1, 0) as usize,
                Instruction::NOP(_) => self.state.acc += 0,
            }
            self.state.pc = (self.state.pc + 1) as usize;
            Some(self.state.clone())
        }
    }
}

impl Program {
    fn new(instr: Vec<Instruction>) -> Program {
        Program {
            state: State::new(),
            instr: instr,
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
