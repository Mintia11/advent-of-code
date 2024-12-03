use shared::uint;

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut lines = [const { Vec::new() }; 2];
        for line in s.lines() {
            let &[first, second, ..] = line.split_whitespace().collect::<Vec<_>>().as_slice()
            else {
                unreachable!()
            };

            lines[0].push(uint(first));
            lines[1].push(uint(second));
        }

        lines[0].sort();
        lines[1].sort();

        lines
    });

    shared::solution_fn(1, &inputs, 11, |lines| {
        (0..lines[0].len())
            .map(|i| shared::dist(lines[0][i], lines[1][i]))
            .sum()
    });

    shared::solution_fn(2, &inputs, 31, |lines| {
        (0..lines[0].len())
            .map(|i| {
                (0..lines[1].len())
                    .map(|j| lines[0][i] == lines[1][j])
                    .map(|r| r as usize)
                    .sum::<usize>()
                    * lines[0][i]
            })
            .sum()
    });
}

shared::runner!();
