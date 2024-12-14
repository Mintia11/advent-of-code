use hashbrown::HashSet;
use shared::two_dimensional_get;
use std::collections::VecDeque;

fn main() {
    let inputs = shared::parse_input(|s| {
        let chars = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut regions = Vec::new();
        let mut counted: HashSet<(usize, usize)> = HashSet::new();

        for (y, row) in chars.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if counted.contains(&(x, y)) {
                    continue;
                }

                let mut region = HashSet::new();
                region.insert((x, y));

                let mut neighbors = VecDeque::new();
                neighbors.push_back((x + 1, y));
                neighbors.push_back((x, y + 1));
                neighbors.push_back((x, y.wrapping_sub(1)));
                neighbors.push_back((x.wrapping_sub(1), y));

                while let Some(neighbor_c) = neighbors.pop_front() {
                    if counted.contains(&neighbor_c) {
                        continue;
                    }

                    let neighbor = two_dimensional_get(&chars, neighbor_c.0, neighbor_c.1);

                    if let Some(neighbor) = neighbor {
                        if neighbor == col {
                            region.insert(neighbor_c);

                            neighbors.push_back((neighbor_c.0 + 1, neighbor_c.1));
                            neighbors.push_back((neighbor_c.0, neighbor_c.1 + 1));
                            neighbors.push_back((neighbor_c.0, neighbor_c.1.wrapping_sub(1)));
                            neighbors.push_back((neighbor_c.0.wrapping_sub(1), neighbor_c.1));

                            counted.insert(neighbor_c);
                        }
                    }
                }

                counted.insert((x, y));
                regions.push(region);
            }
        }

        regions
    });

    shared::solution_fn(1, &inputs, 1930, |input| {
        let mut sum = 0;

        for region in input {
            let area = region.len();

            let mut perimeter = 0;
            if region.len() == 1 {
                sum += 4;

                continue;
            }

            for &(x, y) in &region {
                let possible_neighbors = [
                    (x + 1, y),
                    (x, y + 1),
                    (x, y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y),
                ];

                let mut n = 0;
                for neighbor in possible_neighbors {
                    if region.contains(&neighbor) {
                        n += 1;
                    }
                }

                perimeter += 4 - n;
            }

            sum += area * perimeter;
        }

        sum
    });

    shared::solution_fn(2, &inputs, 1206, |input| {
        let corners = vec![
            ((0, 1), (1, 0), (1, 1)),
            ((1, 0), (0, -1), (1, -1)),
            ((0, -1), (-1, 0), (-1, -1)),
            ((-1, 0), (0, 1), (-1, 1)),
        ];

        let mut sum = 0;

        for region in input {
            if region.len() == 1 {
                sum += 4;

                continue;
            }

            let mut sides = 0;

            for d in &region {
                for &((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &corners {
                    if !region
                        .contains(&(d.0.wrapping_add_signed(dx0), d.1.wrapping_add_signed(dy0)))
                        && !region
                            .contains(&(d.0.wrapping_add_signed(dx1), d.1.wrapping_add_signed(dy1)))
                    {
                        sides += 1;
                        continue;
                    }

                    if region
                        .contains(&(d.0.wrapping_add_signed(dx0), d.1.wrapping_add_signed(dy0)))
                        && region
                            .contains(&(d.0.wrapping_add_signed(dx1), d.1.wrapping_add_signed(dy1)))
                        && !region
                            .contains(&(d.0.wrapping_add_signed(dx2), d.1.wrapping_add_signed(dy2)))
                    {
                        sides += 1;
                        continue;
                    }
                }
            }

            sum += sides * region.len();
        }

        sum
    });
}

shared::runner!();
