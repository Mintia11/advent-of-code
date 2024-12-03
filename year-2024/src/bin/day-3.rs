use regex::Regex;
use shared::uint;

const MUL_REGEX: &str = r"mul\(([0-9]+),([0-9]+)\)";
const DO_REGEX: &str = r"do\(\)";
const DONT_REGEX: &str = r"don\'t\(\)";

fn main() {
    let inputs = shared::parse_input(<str as ToString>::to_string);

    let mul_regex = Regex::new(MUL_REGEX).unwrap();
    let do_regex = Regex::new(DO_REGEX).unwrap();
    let dont_regex = Regex::new(DONT_REGEX).unwrap();

    shared::solution_fn(1, &inputs, 161, |input| {
        mul_regex
            .captures_iter(&input)
            .map(|cap| uint(&cap[1]) * uint(&cap[2]))
            .sum()
    });

    shared::solution_fn(2, &inputs, 48, |input| {
        enum Instruction {
            Do,
            No,
            Mul(usize),
        }

        let does = do_regex
            .captures_iter(&input)
            .map(|cap| cap.get(0).map(|c| c.start()))
            .flatten()
            .collect::<Vec<_>>();

        let donts = dont_regex
            .captures_iter(&input)
            .map(|cap| cap.get(0).map(|c| c.start()))
            .flatten()
            .collect::<Vec<_>>();

        let muls = mul_regex
            .captures_iter(&input)
            .map(|cap| {
                (
                    cap.get(0).map(|c| c.start()).unwrap(),
                    uint(&cap[1]) * uint(&cap[2]),
                )
            })
            .collect::<Vec<_>>();

        let mut instructions = Vec::with_capacity(muls.len() + does.len() + donts.len());
        for (idx, mul) in muls {
            instructions.push((idx, Instruction::Mul(mul)));
        }
        for idx in does {
            instructions.push((idx, Instruction::Do));
        }
        for idx in donts {
            instructions.push((idx, Instruction::No));
        }

        instructions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut sum = 0;
        let mut add = true;
        for (_, instr) in instructions {
            match instr {
                Instruction::Mul(a) => {
                    if add {
                        sum += a;
                    }
                }
                Instruction::Do => add = true,
                Instruction::No => add = false,
            }
        }

        sum
    });
}

shared::runner!();
