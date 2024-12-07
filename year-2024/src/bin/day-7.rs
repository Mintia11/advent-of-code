use shared::uint;

#[derive(Clone)]
pub struct Input {
    result: usize,
    args: Vec<usize>,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    pub fn execute(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
            Self::Concat => lhs * 10usize.pow(((rhs as f64).log10().floor() + 1.) as u32) + rhs,
        }
    }

    pub fn permutations<const CONCAT: bool>(n: usize) -> Vec<Vec<Self>> {
        if n == 0 {
            return vec![vec![]];
        }

        let smaller_permutations = Self::permutations::<CONCAT>(n - 1);
        let mut result = Vec::new();

        for perm in smaller_permutations {
            let mut with_add = perm.clone();
            with_add.push(Self::Add);
            result.push(with_add);

            let mut with_multiply = perm.clone();
            with_multiply.push(Self::Multiply);
            result.push(with_multiply);

            if CONCAT {
                let mut with_concat = perm.clone();
                with_concat.push(Self::Concat);
                result.push(with_concat);
            }
        }

        result
    }
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut out = Vec::with_capacity(s.lines().count());

        for line in s.lines() {
            let (result, args) = line.split_once(':').unwrap();

            let result = uint(result);

            let args = args.trim();
            let args = args.split_ascii_whitespace().map(uint).collect::<Vec<_>>();

            out.push(Input { result, args })
        }

        out
    });

    shared::solution_fn(1, &inputs, 3749, |input| {
        let mut sum = 0;

        'next_line: for line in input {
            let Input { result, args } = line;

            let operators = Operator::permutations::<false>(args.len() - 1);
            'next_op: for line in operators {
                let mut calc_result = args[0];

                for (i, operator) in line.iter().enumerate() {
                    calc_result = operator.execute(calc_result, args[i + 1]);
                }

                if result == calc_result {
                    sum += result;
                    continue 'next_line;
                }

                if calc_result > result {
                    continue 'next_op;
                }
            }
        }

        sum
    });

    shared::solution_fn(2, &inputs, 11387, |input| {
        let mut sum = 0;

        'next_line: for line in input {
            let Input { result, args } = line;

            let operators = Operator::permutations::<true>(args.len() - 1);
            'next_op: for line in operators {
                let mut calc_result = args[0];

                for (i, operator) in line.iter().enumerate() {
                    calc_result = operator.execute(calc_result, args[i + 1]);
                }

                if result == calc_result {
                    sum += result;
                    continue 'next_line;
                }

                if calc_result > result {
                    continue 'next_op;
                }
            }
        }

        sum
    });
}

shared::runner!();
