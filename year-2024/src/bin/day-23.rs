use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut out: HashMap<String, HashSet<String>> = HashMap::new();

        for line in s.lines() {
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (a.to_string(), b.to_string());

            out.entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            out.entry(b.clone())
                .or_insert_with(HashSet::new)
                .insert(a.clone());
        }

        out
    });

    shared::solution_fn(1, &inputs, 7, |input| {
        let mut sets = HashSet::new();

        for (n1, edges) in &input {
            if n1.starts_with('t') {
                for n2 in edges.iter() {
                    for n3 in input[n2].intersection(edges) {
                        let mut set = [n1.clone(), n2.clone(), n3.clone()];
                        set.sort_unstable();
                        sets.insert(set);
                    }
                }
            }
        }

        sets.len()
    });

    shared::solution_fn(2, &inputs, "co,de,ka,ta".to_string(), |input| {
        let mut seen: HashSet<String> = HashSet::new();
        let mut clique = Vec::new();
        let mut largest = Vec::new();

        for (n1, edges) in &input {
            if !seen.contains(n1) {
                clique.clear();
                clique.push(n1);

                for n2 in edges {
                    let other = &input[n2];

                    if clique.iter().all(|&n3| other.contains(n3)) {
                        seen.insert(n2.clone());
                        clique.push(n2);
                    }
                }

                if clique.len() > largest.len() {
                    largest.clone_from(&clique);
                }
            }
        }

        largest.sort_unstable();
        largest.iter().join(",")
    });
}

shared::runner!();
