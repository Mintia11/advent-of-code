use std::collections::BinaryHeap;

use hashbrown::{HashMap, HashSet};
use shared::{dist, internal::is_running_sample, uint};

#[derive(PartialEq, Eq)]
struct Node {
    coords: (usize, usize),
    priority: usize,
}

impl Node {
    pub fn neighbors(&self) -> [(usize, usize); 4] {
        let (x, y) = self.coords;

        [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ]
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dist_x = dist(a.0, b.0);
    let dist_y = dist(a.1, b.1);

    dist_x + dist_y
}

fn a_star(
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
    len: usize,
) -> Option<Vec<(usize, usize)>> {
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        coords: start,
        priority: 0,
    });

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();

    costs.insert(start, 0);

    while let Some(current) = queue.pop() {
        if current.coords == end {
            let mut path = vec![];
            let mut current = Some(&current.coords);

            while let Some(c) = current {
                path.push(*c);
                current = came_from.get(c);
            }

            path.reverse();

            return Some(path);
        }

        for neighbor in current.neighbors() {
            if walls.contains(&neighbor) {
                continue;
            }

            if neighbor.0 > len || neighbor.1 > len {
                continue;
            }

            let new_cost = costs[&current.coords] + heuristic(current.coords, neighbor);
            if !costs.contains_key(&neighbor) || new_cost < costs[&neighbor] {
                costs.insert(neighbor, new_cost);

                queue.push(Node {
                    coords: neighbor,
                    priority: new_cost + heuristic(neighbor, end),
                });
                came_from.insert(neighbor, current.coords);
            }
        }
    }

    None
}

fn main() {
    let inputs = shared::parse_input(|s| {
        s.lines()
            .flat_map(|l| l.split_once(',').map(|(a, b)| (uint(a), uint(b))))
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 22, |input| {
        let end = if is_running_sample() { 6 } else { 70 };

        let walls = input[..if is_running_sample() { 12 } else { 1024 }]
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let path = a_star((0, 0), (end, end), walls.clone(), end).unwrap();

        path.len() - 1
    });

    shared::solution_fn(2, &inputs, "6,1".to_string(), |input| {
        let end = if is_running_sample() { 6 } else { 70 };

        for i in if is_running_sample() { 12 } else { 1024 }..input.len() {
            let walls = input[..i].iter().copied().collect::<HashSet<_>>();
            let path = a_star((0, 0), (end, end), walls.clone(), end);

            if let None = path {
                return format!("{},{}", input[i - 1].0, input[i - 1].1);
            }
        }

        panic!("How")
    });
}

shared::runner!();
