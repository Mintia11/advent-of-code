use nalgebra::Matrix2;
use regex::Regex;
use shared::uint;

#[derive(Default, Debug, Clone, Copy)]
struct Input {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn calc_price<const PART2: bool>(
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
) -> usize {
    // prize.x = x * button_a.x + y * button_b.x
    // prize.y = x * button_a.y + y * button_b.y

    let d = Matrix2::new(
        button_a.0 as f64,
        button_b.0 as f64,
        button_a.1 as f64,
        button_b.1 as f64,
    );
    let d = d.determinant().floor() as isize;

    let dx = Matrix2::new(
        prize.0 as f64,
        button_b.0 as f64,
        prize.1 as f64,
        button_b.1 as f64,
    );
    let dx = dx.determinant().floor() as isize;

    let dy = Matrix2::new(
        button_a.0 as f64,
        prize.0 as f64,
        button_a.1 as f64,
        prize.1 as f64,
    );
    let dy = dy.determinant().floor() as isize;

    if dx % d != 0 || dy % d != 0 {
        return 0;
    }

    let x = dx / d;
    let y = dy / d;

    if !PART2 && (x > 100 || y > 100) {
        return 0;
    }

    if x.is_negative() || y.is_negative() {
        return 0;
    }

    let x = x as usize;
    let y = y as usize;

    if x * button_a.0 + y * button_b.0 != prize.0 || x * button_a.1 + y * button_b.1 != prize.1 {
        return 0;
    }

    (x * 3) + y
}

fn main() {
    let button_regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let inputs = shared::parse_input(|s| {
        let mut out = Vec::new();

        let mut input = Input::default();
        for line in s.lines() {
            let button = button_regex.captures(line);
            if let Some(button) = button {
                let right = match &button[1] {
                    "A" => &mut input.button_a,
                    "B" => &mut input.button_b,
                    _ => unreachable!(),
                };

                right.0 = uint(&button[2]);
                right.1 = uint(&button[3]);

                continue;
            }

            let prize = prize_regex.captures(line);
            if let Some(prize) = prize {
                input.prize.0 = uint(&prize[1]);
                input.prize.1 = uint(&prize[2]);

                out.push(input);
                input = Input::default();
            }
        }

        out
    });

    shared::solution_fn(1, &inputs, 480, |input| {
        input
            .iter()
            .map(
                |&Input {
                     button_a,
                     button_b,
                     prize,
                 }| calc_price::<false>(button_a, button_b, prize),
            )
            .sum()
    });

    shared::solution_fn(2, &inputs, 875318608908, |input| {
        input
            .iter()
            .map(
                |&Input {
                     button_a,
                     button_b,
                     prize,
                 }| {
                    calc_price::<true>(
                        button_a,
                        button_b,
                        (prize.0 + 10000000000000, prize.1 + 10000000000000),
                    )
                },
            )
            .sum()
    });
}

shared::runner!();
