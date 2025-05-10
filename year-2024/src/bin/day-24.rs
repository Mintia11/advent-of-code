#![allow(unused_attributes)]
#![feature(let_chains)]

use hashbrown::HashMap;
use regex::Regex;
use shared::internal::is_running_sample;

#[derive(Clone, Debug)]
enum Opcode {
    And,
    Or,
    Xor,
}

impl Opcode {
    pub fn exec(&self, op1: bool, op2: bool) -> bool {
        match self {
            Self::And => op1 & op2,
            Self::Or => op1 | op2,
            Self::Xor => op1 ^ op2,
        }
    }
}

impl TryFrom<&str> for Opcode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err("Unknown operator"),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    op1: String,
    opcode: Opcode,
    op2: String,
    out: String,

    executed: bool,
}

#[derive(Clone, Debug)]
struct Input {
    regs: HashMap<String, bool>,
    instructions: Vec<Instruction>,
}

fn main() {
    let val_regex = Regex::new(r"([xy]\d+): (0|1)").unwrap();
    let instr_regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();

    let inputs = shared::parse_input(|s| {
        let mut regs = HashMap::new();
        let mut instructions = Vec::new();

        for line in s.lines() {
            if let Some(val) = val_regex.captures(line) {
                regs.insert(
                    val[1].to_string(),
                    val[2].parse::<usize>().map(|v| v == 1).unwrap(),
                );

                continue;
            }

            if let Some(instr) = instr_regex.captures(line) {
                instructions.push(Instruction {
                    op1: instr[1].to_string(),
                    opcode: instr[2].try_into().unwrap(),
                    op2: instr[3].to_string(),
                    out: instr[4].to_string(),

                    executed: false,
                });
            }
        }

        Input { regs, instructions }
    });

    shared::solution_fn(1, &inputs, 2024, |mut input| {
        while !input.instructions.iter().all(|i| i.executed) {
            for instr in input.instructions.iter_mut() {
                if let Some(op1) = input.regs.get(&instr.op1).cloned()
                    && let Some(op2) = input.regs.get(&instr.op2).cloned()
                {
                    let out = instr.opcode.exec(op1, op2);
                    input.regs.insert(instr.out.clone(), out);
                    instr.executed = true;
                }
            }
        }

        let mut out = 0;

        for (reg, val) in input.regs {
            if let Some(idx) = reg.strip_prefix('z') {
                let idx = idx.parse::<usize>().unwrap();

                out |= (val as usize) << idx;
            }
        }

        out
    });

    shared::solution_fn(2, &inputs, "view out.dot".to_string(), |input| {
        if is_running_sample() {
            return "view out.dot".to_string();
        }

        use std::fmt::Write;
        let mut out = String::new();

        writeln!(out, "digraph G {{").unwrap();
        for instr in &input.instructions {
            let id = format!("{}_{}_{}", instr.op1, instr.op2, instr.out);
            writeln!(out, "\t{id} [label=\"{:?}\", shape=box]", instr.opcode).unwrap();
            writeln!(
                out,
                "\t{} [shape=diamond{}]",
                instr.out,
                if instr.out.contains('z') {
                    ",color=red"
                } else {
                    ""
                }
            )
            .unwrap();

            writeln!(out, "\t{} -> {}", instr.op1, id).unwrap();
            writeln!(out, "\t{} -> {}", instr.op2, id).unwrap();
            writeln!(out, "\t{} -> {}", id, instr.out).unwrap();
        }
        // writeln!(out, "\tsubgraph outputs {{").unwrap();
        // writeln!(out, "\t\trank=\"same\"").unwrap();
        // for instr in &input.instructions {
        //     if instr.out.starts_with('z') {
        //         writeln!(out, "\t\t{} [shape=diamond]", instr.out).unwrap();
        //     }
        // }
        // writeln!(out, "\t}}").unwrap();
        writeln!(out, "}}").unwrap();

        // todo!("Manual solution using graphviz");

        // std::fs::write("out.dot", out).unwrap();

        "view out.dot".to_string()
    });
}

shared::runner!();
