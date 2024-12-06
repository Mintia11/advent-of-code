#![allow(unused_attributes)]
#![feature(let_chains)]

use hashbrown::HashSet;

use shared::{two_dimensional_find, two_dimensional_get};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&mut self) {
        match self {
            Direction::Top => *self = Direction::Right,
            Direction::Right => *self = Direction::Bottom,
            Direction::Bottom => *self = Direction::Left,
            Direction::Left => *self = Direction::Top,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapEntry {
    Obstacle,
    Empty,
    Guard(Direction),
}

enum Action {
    Obstacle,
    Continue,
    Escape,
}

fn walk(map: &[Vec<MapEntry>], guard: &mut (usize, usize), direction: &mut Direction) -> Action {
    let saved_pos = *guard;

    match direction {
        Direction::Top => {
            let new_y = guard.1.wrapping_sub(1);
            guard.1 = new_y;
        }
        Direction::Right => {
            guard.0 += 1;
        }
        Direction::Bottom => {
            guard.1 += 1;
        }
        Direction::Left => {
            let new_x = guard.0.wrapping_sub(1);
            guard.0 = new_x;
        }
    }

    if let Some(MapEntry::Obstacle) = two_dimensional_get(map, guard.0, guard.1) {
        *guard = saved_pos;
        direction.rotate();

        return Action::Obstacle;
    } else if let None = two_dimensional_get(map, guard.0, guard.1) {
        return Action::Escape;
    }

    Action::Continue
}

fn visited_positions(
    map: &[Vec<MapEntry>],
    mut guard: (usize, usize),
    mut direction: Direction,
) -> HashSet<(usize, usize)> {
    let mut positions: HashSet<(usize, usize)> = HashSet::default();
    positions.insert(guard);

    while let action = walk(&map, &mut guard, &mut direction)
        && matches!(action, Action::Continue | Action::Obstacle)
    {
        match action {
            Action::Continue => {
                positions.insert(guard);
            }
            Action::Obstacle => {}
            Action::Escape => unreachable!(),
        }
    }

    positions
}

fn is_looping(map: &[Vec<MapEntry>], mut guard: (usize, usize), mut direction: Direction) -> bool {
    let mut positions: HashSet<((usize, usize), Direction)> = HashSet::default();

    while let action = walk(&map, &mut guard, &mut direction)
        && matches!(action, Action::Continue | Action::Obstacle)
    {
        match action {
            Action::Continue => {
                if positions.contains(&(guard, direction)) {
                    return true;
                }

                positions.insert((guard, direction));
            }
            Action::Obstacle => {}
            Action::Escape => unreachable!(),
        }
    }

    false
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut out = Vec::with_capacity(s.lines().count());

        for line in s.lines() {
            let mut col = Vec::with_capacity(line.len());

            for chr in line.chars() {
                col.push(match chr {
                    '.' => MapEntry::Empty,
                    '^' => MapEntry::Guard(Direction::Top),
                    '#' => MapEntry::Obstacle,
                    _ => unreachable!("Invalid character {:?} in input", chr),
                });
            }

            out.push(col);
        }

        out
    });

    shared::solution_fn(1, &inputs, 41, |input| {
        let guard =
            two_dimensional_find(&input, MapEntry::Guard(Direction::Top)).expect("Guard not found");

        let positions = visited_positions(&input, guard, Direction::Top);

        positions.len()
    });

    shared::solution_fn(2, &inputs, 6, |mut input| {
        let mut loops = 0;

        let guard =
            two_dimensional_find(&input, MapEntry::Guard(Direction::Top)).expect("Guard not found");

        let visited_positions = visited_positions(&input, guard, Direction::Top);

        for (x, y) in visited_positions {
            let mut old_entry = MapEntry::Obstacle;
            if let MapEntry::Guard(_) = input[y][x] {
                continue;
            }

            std::mem::swap(&mut input[y][x], &mut old_entry);

            if is_looping(&input, guard, Direction::Top) {
                loops += 1;
            }

            input[y][x] = old_entry;
        }

        loops
    });
}

shared::runner!();
