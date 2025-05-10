fn main() {
    let inputs = shared::parse_input(|s| s.lines().map(ToString::to_string).collect::<Vec<_>>());

    shared::solution_fn(1, &inputs, 1, |input| {
        input.iter().map(|l| false).map(|b| b as usize).sum()
    });

    shared::solution_fn(2, &inputs, todo!("Part 2 sample solution"), |input| {
        todo!("Part 2")
    });
}

shared::runner!();
