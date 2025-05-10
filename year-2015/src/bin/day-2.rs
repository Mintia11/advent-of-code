use shared::uint;

fn main() {
    let inputs = shared::parse_input(|s| {
        s.lines()
            .map(|l| {
                l.split('x')
                    .map(uint)
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
            })
            .collect::<Result<Vec<[usize; 3]>, _>>()
            .unwrap()
    });

    shared::solution_fn(1, &inputs, 58 + 43, |input| {
        input
            .iter()
            .map(|dim| {
                let mut shortest = dim.clone();
                shortest.sort_unstable();

                2 * dim[0] * dim[1]
                    + 2 * dim[1] * dim[2]
                    + 2 * dim[2] * dim[0]
                    + shortest[0] * shortest[1]
            })
            .sum()
    });

    shared::solution_fn(2, &inputs, 34 + 14, |input| {
        input
            .iter()
            .map(|dim| {
                let mut shortest = dim.clone();
                shortest.sort_unstable();

                2 * shortest[0] + 2 * shortest[1] + dim.iter().product::<usize>()
            })
            .sum()
    });
}

shared::runner!();
