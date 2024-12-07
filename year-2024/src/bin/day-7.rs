use shared::uint;

#[derive(Clone)]
pub struct Input {
    result: usize,
    args: Vec<usize>,
}

fn concat(lhs: usize, rhs: usize) -> usize {
    lhs * 10usize.pow(((rhs as f64).log10().floor() + 1.) as u32) + rhs
}

fn resolvable<const CONCAT: bool>(result: usize, current: usize, rest: &[usize]) -> bool {
    if current > result {
        return false;
    }

    let Some(&next) = rest.get(0) else {
        return result == current;
    };

    resolvable::<CONCAT>(result, current + next, &rest[1..])
        || resolvable::<CONCAT>(result, current * next, &rest[1..])
        || if CONCAT {
            resolvable::<CONCAT>(result, concat(current, next), &rest[1..])
        } else {
            false
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

        for line in input {
            let Input { result, args } = line;

            if resolvable::<false>(result, args[0], &args[1..]) {
                sum += result
            }
        }

        sum
    });

    shared::solution_fn(2, &inputs, 11387, |input| {
        let mut sum = 0;

        for line in input {
            let Input { result, args } = line;

            if resolvable::<true>(result, args[0], &args[1..]) {
                sum += result
            }
        }

        sum
    });
}

shared::runner!();
