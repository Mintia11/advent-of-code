use hashbrown::{HashMap, HashSet};
use shared::{dist, internal::is_running_sample};

#[derive(Clone)]
struct Input {
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let dist_x = dist(a.0, b.0);
    let dist_y = dist(a.1, b.1);

    dist_x + dist_y
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => start = Some((x, y)),
                    'E' => end = Some((x, y)),
                    '#' => {
                        let _ = walls.insert((x, y));
                    }
                    '.' => {}
                    _ => {}
                }
            }
        }

        Input {
            start: start.unwrap(),
            end: end.unwrap(),
            walls,
        }
    });

    shared::solution_fn(1, &inputs, 94, |input| {
        let mut path = HashMap::new();
        path.insert(input.start, 0);
        let mut position = input.start;

        while position != input.end {
            for next_position in [
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
            ] {
                if !path.contains_key(&next_position) && !input.walls.contains(&next_position) {
                    path.insert(next_position, *path.get(&position).unwrap() + 1);
                    position = next_position;
                }
            }
        }

        let mut count = 0;
        for (position, cost) in path.iter() {
            for next_position in [
                (position.0 - 2, position.1),
                (position.0 + 2, position.1),
                (position.0, position.1 - 2),
                (position.0, position.1 + 2),
            ] {
                if let Some(next_cost) = path.get(&next_position) {
                    if next_cost - cost >= if is_running_sample() { 1 } else { 102 } {
                        count += 1;
                    }
                }
            }
        }

        count
    });

    shared::solution_fn(2, &inputs, 0, |input| {
        let mut path = vec![input.start];
        let mut visited = HashSet::new();
        visited.insert(input.start);
        let mut position = input.start;

        while position != input.end {
            for next_position in [
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
            ] {
                if !visited.contains(&next_position) && !input.walls.contains(&next_position) {
                    visited.insert(next_position);
                    path.push(next_position);
                    position = next_position;
                }
            }
        }

        let mut count = 0;

        for cheat_start_index in 0..path.len() {
            for cheat_end_index in cheat_start_index + 1..path.len() {
                let distance = distance(path[cheat_start_index], path[cheat_end_index]);

                if distance <= 20 && cheat_end_index - cheat_start_index >= 100 + distance {
                    count += 1;
                }
            }
        }

        count
    });
}

shared::runner!();
