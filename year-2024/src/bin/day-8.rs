#![allow(unused_attributes)]
#![feature(let_chains)]

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Clone)]
struct Input {
    map: HashMap<(isize, isize), char>,
    width: isize,
    height: isize,
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut out = HashMap::new();

        let width = s.lines().nth(0).unwrap().len() as isize;
        let height = s.lines().count() as isize;

        for (y, line) in s.lines().enumerate() {
            for (x, col) in line.chars().enumerate() {
                if col.is_alphanumeric() {
                    out.insert((x as isize, y as isize), col);
                }
            }
        }

        Input {
            map: out,
            width,
            height,
        }
    });

    shared::solution_fn(1, &inputs, 14, |input| {
        let mut antinodes = HashSet::new();

        for antennas in input.map.iter().combinations(2) {
            let (a1, f1) = antennas[0];
            let (a2, f2) = antennas[1];

            if a1 == a2 {
                continue;
            }

            if f1 != f2 {
                continue;
            }

            let dx = a2.0 - a1.0;
            let dy = a2.1 - a1.1;

            let node1 = (a1.0 - dx, a1.1 - dy);
            let node2 = (a2.0 + dx, a2.1 + dy);

            if (0..input.width).contains(&node1.0) && (0..input.height).contains(&node1.1) {
                antinodes.insert(node1);
            }
            if (0..input.width).contains(&node2.0) && (0..input.height).contains(&node2.1) {
                antinodes.insert(node2);
            }
        }

        antinodes.len()
    });

    shared::solution_fn(2, &inputs, 34, |input| {
        let mut antinodes = HashSet::new();

        for antennas in input.map.iter().combinations(2) {
            let (a1, f1) = antennas[0];
            let (a2, f2) = antennas[1];

            if a1 == a2 {
                continue;
            }

            if f1 != f2 {
                continue;
            }

            let dx = a2.0 - a1.0;
            let dy = a2.1 - a1.1;

            for k in -input.width..input.width {
                let ax = a1.0 + k * dx;
                let ay = a1.1 + k * dy;

                if (0..input.width).contains(&ax) && (0..input.height).contains(&ay) {
                    antinodes.insert((ax, ay));
                }
            }
        }

        antinodes.len()
    });
}

shared::runner!();
