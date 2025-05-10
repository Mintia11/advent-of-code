use hashbrown::HashSet;
use shared::is_crlf;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Input {
    Lock([u8; 5]),
    Key([u8; 5]),
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let entries = s
            .split(if is_crlf(s) { "\r\n\r\n" } else { "\n\n" })
            .collect::<Vec<_>>();

        let mut out = Vec::with_capacity(entries.len());
        for entry in entries {
            let lines = entry.lines().collect::<Vec<_>>();
            let mut cols = [0; 5];

            for &line in &lines {
                for (col, val) in line.chars().enumerate() {
                    if val == '#' {
                        cols[col] += 1;
                    }
                }
            }

            if lines[0] == "....." {
                out.push(Input::Key(cols.map(|c| c - 1)));
            } else if lines[0] == "#####" {
                out.push(Input::Lock(cols.map(|c| c - 1)));
            } else {
                unreachable!()
            }
        }

        out
    });

    shared::solution_fn(1, &inputs, 3, |input| {
        let mut out = 0;
        let mut lock_key_pairs: HashSet<(&Input, &Input)> = HashSet::new();

        for _lock in input.iter().filter(|i| matches!(i, Input::Lock(_))) {
            for _key in input.iter().filter(|i| matches!(i, Input::Key(_))) {
                if lock_key_pairs.contains(&(_lock, _key)) {
                    continue;
                }

                let Input::Lock(lock) = _lock else {
                    unreachable!()
                };
                let Input::Key(key) = _key else {
                    unreachable!()
                };

                let sum: [u8; 5] = core::array::from_fn(|n| lock[n] + key[n]);
                let wrong = sum.iter().fold(false, |acc, &s| (s > 5) || acc);

                if !wrong {
                    out += 1;
                }

                lock_key_pairs.insert((_lock, _key));
            }
        }

        out
    });

    shared::solution_fn(2, &inputs, 0, |_| 0);
}

shared::runner!();
