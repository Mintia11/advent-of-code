fn main() {
    let inputs = shared::parse_input(|s| s.chars().collect::<Vec<_>>());

    shared::solution_fn(1, &inputs, -2, |input| {
        let mut count = 0;

        for chr in input {
            match chr {
                '(' => count += 1,
                ')' => count -= 1,
                _ => unreachable!(),
            }
        }

        count
    });

    shared::solution_fn(2, &inputs, 1, |input| {
        let mut count: i32 = 0;

        for (chr, i) in input.iter().zip(1..) {
            match chr {
                '(' => count += 1,
                ')' => count -= 1,
                _ => unreachable!(),
            }

            if count.is_negative() {
                return i;
            }
        }

        0
    });
}

shared::runner!();
