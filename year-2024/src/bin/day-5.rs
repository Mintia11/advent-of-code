use shared::{is_crlf, uint};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Clone, Debug)]
struct Input {
    update_ordering: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn sort_function(
    ordering: &HashSet<(usize, usize)>,
) -> impl Fn(&usize, &usize) -> Ordering + use<'_> {
    move |&a, &b| {
        if ordering.contains(&(a, b)) {
            Ordering::Less
        } else if ordering.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let (order, updates) = s
            .split_once(if is_crlf(s) { "\r\n\r\n" } else { "\n\n" })
            .unwrap();

        let ordering = order
            .lines()
            .filter_map(|line| line.split_once('|').map(|(a, b)| (uint(a), uint(b))))
            .collect();

        let updates = updates
            .lines()
            .map(|line| line.split(',').map(uint).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Input {
            update_ordering: ordering,
            updates,
        }
    });

    shared::solution_fn(1, &inputs, 143, |input| {
        let mut updates = input.updates.clone();
        updates
            .iter_mut()
            .for_each(|update| update.sort_unstable_by(sort_function(&input.update_ordering)));

        input
            .updates
            .iter()
            .zip(updates.iter())
            .filter(|&(original, sorted)| (sorted == original))
            .map(|(original, _)| original[original.len() / 2])
            .sum()
    });

    shared::solution_fn(2, &inputs, 123, |input| {
        let mut updates = input.updates.clone();
        updates
            .iter_mut()
            .for_each(|update| update.sort_unstable_by(sort_function(&input.update_ordering)));

        input
            .updates
            .iter()
            .zip(updates.iter())
            .filter(|&(original, sorted)| (sorted != original))
            .map(|(_, sorted)| sorted[sorted.len() / 2])
            .sum()
    });
}

shared::runner!();
