use hashbrown::HashMap;
use shared::uint;

fn count_digit(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut count = 0;
    while n != 0 {
        n /= 10;

        count += 1;
    }

    count
}

fn blink(stone_counts: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::with_capacity(stone_counts.len());

    for (&stone, &count) in stone_counts.iter() {
        let mut add = |n| {
            *counts.entry(n).or_insert(0) += count;
        };

        if stone == 0 {
            add(1);
        } else {
            let digits = count_digit(stone);

            if digits % 2 == 0 {
                let left = stone / (10usize.pow(digits as u32 / 2));
                let right = stone % (10usize.pow(digits as u32 / 2));

                add(left);
                add(right);
            } else {
                add(stone * 2024);
            }
        }
    }

    counts
}

fn main() {
    let inputs = shared::parse_input(|s| {
        s.split_whitespace()
            .map(uint)
            .map(|s| (s, 1))
            .collect::<HashMap<_, _>>()
    });

    shared::solution_fn(1, &inputs, 55312, |mut input| {
        for _ in 0..25 {
            input = blink(&input);
        }

        input.values().sum()
    });

    shared::solution_fn(2, &inputs, 65601038650482, |mut input| {
        for _ in 0..75 {
            input = blink(&input);
        }

        input.values().sum()
    });
}

shared::runner!();
