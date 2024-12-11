#![allow(unused_attributes)]
#![feature(let_chains)]

use hashbrown::HashSet;
use shared::{two_dimensional_get, uint_char};

fn walk(
    map: &[Vec<usize>],
    pos: (usize, usize),
    visited: &mut Option<HashSet<(usize, usize)>>,
) -> usize {
    let current = two_dimensional_get(map, pos.0, pos.1);
    if let Some(9) = current {
        if let Some(ref mut visited) = visited
            && !visited.contains(&pos)
        {
            visited.insert(pos);

            return 1;
        } else if let None = visited {
            return 1;
        }
    }

    let current = current.unwrap();
    let mut score = 0;

    let top = two_dimensional_get(map, pos.0, pos.1.wrapping_sub(1));
    if let Some(top) = top {
        if top == current + 1 {
            score += walk(map, (pos.0, pos.1.wrapping_sub(1)), visited);
        }
    }

    let right = two_dimensional_get(map, pos.0.wrapping_add(1), pos.1);
    if let Some(right) = right {
        if right == current + 1 {
            score += walk(map, (pos.0.wrapping_add(1), pos.1), visited);
        }
    }

    let bottom = two_dimensional_get(map, pos.0, pos.1.wrapping_add(1));
    if let Some(bottom) = bottom {
        if bottom == current + 1 {
            score += walk(map, (pos.0, pos.1.wrapping_add(1)), visited);
        }
    }

    let left = two_dimensional_get(map, pos.0.wrapping_sub(1), pos.1);
    if let Some(left) = left {
        if left == current + 1 {
            score += walk(map, (pos.0.wrapping_sub(1), pos.1), visited);
        }
    }

    return score;
}

fn main() {
    let inputs = shared::parse_input(|s| {
        s.lines()
            .map(|l| l.chars().map(uint_char).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 36, |input| {
        let mut score = 0;

        for (y, row) in input.iter().enumerate() {
            for (x, &a) in row.iter().enumerate() {
                if a == 0 {
                    let to_add = walk(&input, (x, y), &mut Some(HashSet::new()));
                    score += to_add;
                }
            }
        }

        score
    });

    shared::solution_fn(2, &inputs, 81, |input| {
        let mut score = 0;

        for (y, row) in input.iter().enumerate() {
            for (x, &a) in row.iter().enumerate() {
                if a == 0 {
                    let to_add = walk(&input, (x, y), &mut None);
                    score += to_add;
                }
            }
        }

        score
    });
}

shared::runner!();
