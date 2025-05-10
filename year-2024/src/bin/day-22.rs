#![allow(unused_attributes)]
#![feature(portable_simd)]

use std::simd::{num::SimdUint, u64x8};

use hashbrown::{HashMap, HashSet};
use shared::uint;

fn rand(mut seed: usize) -> usize {
    seed ^= (seed << 6) & 0xFFFFFF;
    seed ^= (seed >> 5) & 0xFFFFFF;
    seed ^= (seed << 11) & 0xFFFFFF;
    seed
}

fn rand_simd(mut seeds: u64x8) -> u64x8 {
    seeds ^= (seeds << 6) & u64x8::splat(0xFFFFFF);
    seeds ^= (seeds >> 5) & u64x8::splat(0xFFFFFF);
    seeds ^= (seeds << 11) & u64x8::splat(0xFFFFFF);
    seeds
}

fn main() {
    let inputs = shared::parse_input(|s| s.lines().map(|l| uint(l) as u64).collect::<Vec<_>>());

    shared::solution_fn(1, &inputs, 37327623, |input| {
        input
            .chunks(8)
            .map(|n| {
                (0..2000)
                    .fold(u64x8::load_or_default(n), |acc, _| rand_simd(acc))
                    .reduce_sum()
            })
            .sum()
    });

    shared::solution_fn(2, &inputs, 23, |input| {
        let mut sequences = HashMap::new();
        let mut seen = HashSet::new();

        for buyer in input.iter().map(|&b| b as usize) {
            let mut seed = buyer;
            let mut old_cost = seed % 10;
            let mut deltas = 0;

            for i in 0..2000 {
                seed = rand(seed);
                let cost = seed % 10;

                deltas = ((deltas << 5) & 0xFFFFF) + 10 + cost - old_cost;

                if !seen.contains(&deltas) && i > 3 {
                    seen.insert(deltas);
                    *sequences.entry(deltas).or_insert(0) += cost;
                }

                old_cost = cost;
            }

            seen.clear();
        }

        sequences.values().max().cloned().unwrap()
    });
}

shared::runner!();
