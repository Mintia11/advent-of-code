use std::ops::Add;

use hashbrown::HashSet;
use pathfinding::prelude::astar_bag;
use shared::{dist, two_dimensional_find};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Nothing,
    Start,
    End,
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            '.' => Ok(Self::Nothing),
            _ => Err("Unknown char"),
        }
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    let dist_x = dist(a.0, b.0);
    let dist_y = dist(a.1, b.1);

    dist_x + dist_y
}

fn get_successors(
    pos: (usize, usize),
    dir: Direction,
    walls: &HashSet<(usize, usize)>,
) -> Vec<(((usize, usize), Direction), usize)> {
    let mut successors = vec![];

    let ahead = pos + dir;
    if !walls.contains(&ahead) {
        successors.push(((ahead, dir), 1));
    }

    let right = pos + dir.rotate();
    if !walls.contains(&right) {
        successors.push(((right, dir.rotate()), 1001));
    }

    let behind = pos + dir.rotate().rotate();
    if !walls.contains(&behind) {
        successors.push(((behind, dir.rotate().rotate()), 2001));
    }

    let left = pos + dir.rotate().rotate().rotate();
    if !walls.contains(&left) {
        successors.push(((left, dir.rotate().rotate().rotate()), 1001));
    }

    successors
}

fn main() {
    let inputs = shared::parse_input(|s| {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<Cell>, _>>()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 7036, |input| {
        let start = two_dimensional_find(&input, Cell::Start).unwrap();
        let end = two_dimensional_find(&input, Cell::End).unwrap();

        let walls = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| matches!(c, Cell::Wall).then_some((x, y)))
            .collect::<HashSet<_>>();

        let (_solutions, cost) = astar_bag(
            &(start, Direction::Right),
            |&(pos, dir)| get_successors(pos, dir, &walls),
            |(pos, _)| heuristic(*pos, end),
            |&(pos, _)| pos == end,
        )
        .unwrap();

        cost
    });

    shared::solution_fn(2, &inputs, 45, |input| {
        let start = two_dimensional_find(&input, Cell::Start).unwrap();
        let end = two_dimensional_find(&input, Cell::End).unwrap();

        let walls = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| matches!(c, Cell::Wall).then_some((x, y)))
            .collect::<HashSet<_>>();

        let (solutions, _) = astar_bag(
            &(start, Direction::Right),
            |&(pos, dir)| get_successors(pos, dir, &walls),
            |(pos, _)| heuristic(*pos, end),
            |&(pos, _)| pos == end,
        )
        .unwrap();

        solutions
            .fold(HashSet::new(), |mut acc: HashSet<(usize, usize)>, r| {
                acc.extend(r.iter().map(|&(pos, ..)| pos));
                acc
            })
            .len()
    });
}

shared::runner!();
