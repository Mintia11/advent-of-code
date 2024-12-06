use shared::{concat, uint};

fn is_safe(line: &[usize]) -> bool {
    if !line.is_sorted() && !line.is_sorted_by(|a, b| b < a) {
        return false;
    }

    for i in 0..line.len() {
        if let Some(&b) = line.get(i + 1) {
            let dist = shared::dist(line[i], b);

            if !(1..=3).contains(&dist) {
                return false;
            }
        }
    }

    true
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut data = Vec::new();

        for line in s.lines() {
            let mut inner = Vec::with_capacity(line.len() / 2);

            for n in line.split_whitespace() {
                inner.push(uint(n));
            }

            data.push(inner);
        }

        data
    });

    shared::solution_fn(1, &inputs, 2, |input| {
        input
            .iter()
            .map(|line| is_safe(line.as_slice()))
            .map(usize::from)
            .sum()
    });

    shared::solution_fn(2, &inputs, 4, |input| {
        input
            .iter()
            .filter_map(|line| {
                (0..line.len())
                    .map(|i| concat(&line[..i], &line[i + 1..]))
                    .map(|subset| is_safe(&subset))
                    .find(|&i| i)
            })
            .map(usize::from)
            .sum()
    });
}

shared::runner!();
