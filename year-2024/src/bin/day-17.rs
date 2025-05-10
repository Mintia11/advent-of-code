use itertools::Itertools;
use regex::Regex;
use shared::uint;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(usize),
    Bst(ComboOperand),
    Jnz(usize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
    C,
}

#[derive(Clone, Copy, Debug)]
enum ComboOperand {
    Value(usize),
    Register(Register),
}

impl ComboOperand {
    pub fn get(self, input: &State) -> usize {
        match self {
            ComboOperand::Value(val) => val,
            ComboOperand::Register(r) => match r {
                Register::A => input.a,
                Register::B => input.b,
                Register::C => input.c,
            },
        }
    }
}

impl From<usize> for ComboOperand {
    fn from(value: usize) -> Self {
        match value {
            0..4 => ComboOperand::Value(value),
            4 => ComboOperand::Register(Register::A),
            5 => ComboOperand::Register(Register::B),
            6 => ComboOperand::Register(Register::C),
            7 => panic!("Reserved {}", value),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    a: usize,
    b: usize,
    c: usize,

    pc: usize,
    instrs: Vec<Instruction>,

    orig_instrs: Vec<usize>,
}

impl State {
    pub fn exec(&mut self) -> Vec<usize> {
        let mut out = Vec::new();

        self.pc = 0;
        while self.pc < self.instrs.len() {
            let instr = self.instrs[self.pc];

            match instr {
                Instruction::Adv(d) | Instruction::Bdv(d) | Instruction::Cdv(d) => {
                    let denominator = d.get(&self);
                    let numerator = self.a;

                    let out = numerator / 2usize.pow(denominator as _);

                    match instr {
                        Instruction::Adv(_) => self.a = out,
                        Instruction::Bdv(_) => self.b = out,
                        Instruction::Cdv(_) => self.c = out,
                        _ => unreachable!(),
                    };
                }
                Instruction::Bxl(lit) => {
                    self.b = self.b ^ lit;
                }
                Instruction::Bst(v) => {
                    self.b = v.get(&self) % 8;
                }
                Instruction::Jnz(to) => {
                    if self.a != 0 {
                        self.pc = to / 2;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.b = self.b ^ self.c;
                }
                Instruction::Out(v) => {
                    out.push(v.get(&self) % 8);
                }
            }

            self.pc += 1;
        }

        out
    }
}

fn main() {
    let reg_regex = Regex::new(r"Register ([A-C]): (\d+)").unwrap();
    let instr_regex = Regex::new(r"Program: ((?:\d+,?)+)").unwrap();

    let inputs = shared::parse_input(|s| {
        let mut state = State {
            a: 0,
            b: 0,
            c: 0,
            pc: 0,
            instrs: vec![],
            orig_instrs: vec![],
        };

        for line in s.lines() {
            if let Some(reg) = reg_regex.captures(line) {
                match &reg[1] {
                    "A" => state.a = uint(&reg[2]),
                    "B" => state.b = uint(&reg[2]),
                    "C" => state.c = uint(&reg[2]),
                    _ => panic!("Unknown register {}", &reg[1]),
                }
            }

            if let Some(program) = instr_regex.captures(line) {
                let mut instrs = program[1].split(',').map(uint).collect::<Vec<_>>();
                state.orig_instrs = instrs.clone();

                while instrs.len() >= 2 {
                    let opcode = instrs.remove(0);
                    let sec = instrs.remove(0);

                    let instr = match opcode {
                        0 => Instruction::Adv(sec.into()),
                        1 => Instruction::Bxl(sec),
                        2 => Instruction::Bst(sec.into()),
                        3 => Instruction::Jnz(sec),
                        4 => Instruction::Bxc,
                        5 => Instruction::Out(sec.into()),
                        6 => Instruction::Bdv(sec.into()),
                        7 => Instruction::Cdv(sec.into()),
                        _ => unimplemented!("Opcode: {opcode}"),
                    };

                    state.instrs.push(instr);
                }
            }
        }

        state
    });

    shared::solution_fn(
        1,
        &inputs,
        "4,6,3,5,6,3,5,2,1,0".to_string(),
        |mut input| input.exec().iter().map(ToString::to_string).join(","),
    );

    shared::solution_fn(2, &inputs, 117440, |mut input| {
        fn solve(state: &mut State, instrs: &[usize], i: usize, mut n: usize) -> Option<usize> {
            let target = &instrs[i..];
            n *= 8;

            for n in n..n + 8 {
                state.a = n;
                state.b = 0;
                state.c = 0;

                if state.exec() == target {
                    if i == 0 {
                        return Some(n);
                    }

                    if let Some(n) = solve(state, instrs, i - 1, n) {
                        return Some(n);
                    }
                }
            }

            None
        }

        let instrs = input.orig_instrs.clone();

        solve(&mut input, &instrs, instrs.len() - 1, 0).unwrap()
    });
}

shared::runner!();

/*
Bst(Register(A)), Bxl(3), Cdv(Register(B)), Adv(Value(3)), Bxl(5), Bxc, Out(Register(B)), Jnz(0)

LABEL START

B = A % 8    (B = A & 0x7)
B ^= 3
C = A / 2^B  (C = A >> B)
A = A / 2^3  (A >>= 3)
B ^= 5
B ^= C
OUT B % 8
JNZ START
*/
