use std::ops::Add;

use hashbrown::HashMap;
use itertools::Itertools;
use shared::{two_dimensional_find, two_dimensional_get};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Add<Direction> for (usize, usize) {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;

        match rhs {
            Direction::Top => (x, y.wrapping_sub(1)),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x.wrapping_sub(1), y),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Number(usize),
    Direction(Direction),
    Enter,
    None,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '0'..='9' => Cell::Number(((value as u8) - b'0').into()),
            'A' => Cell::Enter,
            _ => unreachable!(),
        }
    }
}

#[rustfmt::skip]
const NUMERIC_KEYPAD: [[Cell; 3]; 4] = [
    [Cell::Number(7), Cell::Number(8), Cell::Number(9)], 
    [Cell::Number(4), Cell::Number(5), Cell::Number(6)], 
    [Cell::Number(1), Cell::Number(2), Cell::Number(3)], 
    [Cell::None,      Cell::Number(0), Cell::Enter]
];

#[rustfmt::skip]
const DIRECTION_KEYPAD: [[Cell; 3]; 2] = [
    [Cell::None,                       Cell::Direction(Direction::Top),    Cell::Enter],
    [Cell::Direction(Direction::Left), Cell::Direction(Direction::Bottom), Cell::Direction(Direction::Right)]
];

fn next_move(
    keypad: &[[Cell; 3]],
    dir: Direction,
    coords: (usize, usize),
) -> Option<(usize, usize)> {
    let next = coords + dir;

    match two_dimensional_get(keypad, next.0, next.1)? {
        Cell::None => None,
        _ => Some(next),
    }
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let dist_x = shared::dist(a.0, b.0);
    let dist_y = shared::dist(a.1, b.1);

    dist_x + dist_y
}

fn traverse(
    keypad: &[[Cell; 3]],
    start: (usize, usize),
    end: (usize, usize),
    depth: usize,
    cache: &mut HashMap<((usize, usize), (usize, usize), usize), usize>,
) -> usize {
    if let Some(&moves) = cache.get(&(start, end, depth)) {
        return moves;
    }

    if depth == 0 {
        return dist(start, end) + 1;
    }

    let mut moves = Vec::new();
    if start.0 > end.0 {
        moves.extend([Direction::Left].repeat(start.0 - end.0));
    } else {
        moves.extend([Direction::Right].repeat(end.0 - start.0));
    }
    if start.1 > end.1 {
        moves.extend([Direction::Top].repeat(start.1 - end.1));
    } else {
        moves.extend([Direction::Bottom].repeat(end.1 - start.1));
    }

    let result = moves
        .iter()
        .permutations(moves.len())
        .filter_map(|moves| {
            let mut current_pos = start;

            for &&direction in &moves {
                current_pos = next_move(keypad, direction, current_pos)?;
            }

            Some(
                [Cell::Enter]
                    .into_iter()
                    .chain(moves.iter().map(|&&m| Cell::Direction(m)))
                    .chain([Cell::Enter])
                    .tuple_windows()
                    .map(|(a, b)| {
                        let a = two_dimensional_find(&DIRECTION_KEYPAD, a).unwrap();
                        let b = two_dimensional_find(&DIRECTION_KEYPAD, b).unwrap();

                        traverse(&DIRECTION_KEYPAD, a, b, depth - 1, cache)
                    })
                    .sum::<usize>(),
            )
        })
        .min()
        .unwrap();

    cache.insert((start, end, depth), result);

    result
}

fn solve(input: Vec<(usize, Vec<(usize, usize)>)>, depth: usize) -> usize {
    let mut cache = HashMap::new();

    input
        .iter()
        .map(|(num, p)| {
            num * [(2, 3)]
                .iter()
                .chain(p.iter())
                .tuple_windows()
                .map(|(&start, &end)| traverse(&NUMERIC_KEYPAD, start, end, depth, &mut cache))
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let inputs = shared::parse_input(|s| {
        s.lines()
            .map(ToString::to_string)
            .map(|l| {
                (
                    l.strip_suffix("A").unwrap().parse::<usize>().unwrap(),
                    l.chars()
                        .map(Cell::from)
                        .map(|c| two_dimensional_find(&NUMERIC_KEYPAD, c))
                        .collect::<Option<Vec<_>>>()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 126384, |input| solve(input, 2));

    shared::solution_fn(2, &inputs, 154115708116294, |input| solve(input, 25));
}

shared::runner!();
