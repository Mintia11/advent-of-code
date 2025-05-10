#![allow(unused_attributes)]
#![feature(let_chains)]

use shared::{two_dimensional_find, two_dimensional_get};

#[derive(Clone, Copy, Debug)]
enum Move {
    Top,
    Right,
    Left,
    Bottom,
}

impl Move {
    pub fn next_coords(&self, current: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Top => (current.0, current.1.wrapping_sub(1)),
            Self::Right => (current.0 + 1, current.1),
            Self::Bottom => (current.0, current.1 + 1),
            Self::Left => (current.0.wrapping_sub(1), current.1),
        }
    }

    pub fn horizzontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Top),
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Bottom),
            _ => Err("Unknown char"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Wall,
    Box,
    Robot,
    Nothing,

    BoxLeft,
    BoxRight,
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            '.' => Ok(Self::Nothing),
            _ => Err("Unknown char"),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Vec<Vec<Cell>>,
    moves: Vec<Move>,
}

fn move_box(map: &mut [Vec<Cell>], dir: Move, coords: (usize, usize)) -> bool {
    let next = dir.next_coords(coords);

    let mut was_able_to_move = true;

    if matches!(
        two_dimensional_get(map, next.0, next.1),
        Some(Cell::Box) | Some(Cell::BoxLeft) | Some(Cell::BoxRight)
    ) {
        if !move_box(map, dir, next) {
            was_able_to_move = false;
        }
    }

    if let Some(Cell::Nothing) = two_dimensional_get(map, next.0, next.1) {
        let this = map[coords.1][coords.0];
        map[next.1][next.0] = this;
        map[coords.1][coords.0] = Cell::Nothing;
    } else {
        was_able_to_move = false;
    }

    was_able_to_move
}

fn adjacent_box(map: &[Vec<Cell>], coords: (usize, usize)) -> (usize, usize) {
    let Some(this) = two_dimensional_get(map, coords.0, coords.1) else {
        unreachable!()
    };

    let other_x = match this {
        Cell::BoxRight => coords.0 - 1,
        Cell::BoxLeft => coords.0 + 1,
        _ => unreachable!("box isn't boxleft or boxright {coords:?} {this:?}"),
    };

    (other_x, coords.1)
}

fn check_move_box_vertical(map: &mut [Vec<Cell>], dir: Move, coords: (usize, usize)) -> bool {
    let Some(this) = two_dimensional_get(map, coords.0, coords.1) else {
        return false;
    };

    match this {
        Cell::BoxLeft | Cell::BoxRight => {
            check_move_box_vertical(map, dir, dir.next_coords(coords))
                && check_move_box_vertical(map, dir, dir.next_coords(adjacent_box(map, coords)))
        }
        Cell::Nothing => true,
        Cell::Wall => false,
        _ => todo!("{:?}", this),
    }
}

fn move_box_vertical(map: &mut [Vec<Cell>], dir: Move, coords: (usize, usize)) -> bool {
    let Some(this) = two_dimensional_get(map, coords.0, coords.1) else {
        return false;
    };

    match this {
        Cell::BoxLeft | Cell::BoxRight => {
            if move_box_vertical(map, dir, dir.next_coords(coords))
                && move_box_vertical(map, dir, dir.next_coords(adjacent_box(map, coords)))
            {
                let other_coords = adjacent_box(map, coords);
                let Some(other) = two_dimensional_get(map, other_coords.0, other_coords.1) else {
                    return false;
                };

                map[coords.1][coords.0] = Cell::Nothing;
                map[other_coords.1][other_coords.0] = Cell::Nothing;

                let next_this = dir.next_coords(coords);
                let next_adj = dir.next_coords(other_coords);

                map[next_this.1][next_this.0] = this;
                map[next_adj.1][next_adj.0] = other;

                true
            } else {
                false
            }
        }
        Cell::Nothing => true,
        Cell::Wall => false,
        _ => todo!("{:?}", this),
    }
}

fn walk<const PART2: bool>(map: &mut [Vec<Cell>], moves: &[Move], mut robot: (usize, usize)) {
    for &mov in moves {
        let next_coords = mov.next_coords(robot);

        if let Some(next) = two_dimensional_get(map, next_coords.0, next_coords.1) {
            match next {
                Cell::Wall => {}
                Cell::Robot | Cell::Nothing => {
                    robot = next_coords;
                }
                Cell::Box => {
                    if move_box(map, mov, next_coords) {
                        robot = next_coords;
                    }
                }
                Cell::BoxLeft | Cell::BoxRight => {
                    if !PART2 {
                        unreachable!()
                    } else {
                        if mov.horizzontal() {
                            if move_box(map, mov, next_coords) {
                                robot = next_coords;
                            }
                        } else {
                            if check_move_box_vertical(map, mov, next_coords) {
                                move_box_vertical(map, mov, next_coords);
                                robot = next_coords;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut map = Vec::new();
        let mut moves = Vec::new();

        let mut do_moves = false;
        for line in s.lines() {
            if do_moves {
                moves.extend(
                    line.chars()
                        .map(TryInto::<Move>::try_into)
                        .map(Result::unwrap),
                );
            } else {
                if line == "" {
                    do_moves = true;
                } else {
                    map.push(
                        line.chars()
                            .map(TryInto::try_into)
                            .collect::<Result<Vec<Cell>, _>>()
                            .unwrap(),
                    );
                }
            }
        }

        Input { map, moves }
    });

    shared::solution_fn(1, &inputs, 10092, |mut input| {
        let robot = two_dimensional_find(&input.map, Cell::Robot).expect("Cannot find robot");
        input.map[robot.1][robot.0] = Cell::Nothing;

        walk::<false>(&mut input.map, &input.moves, robot);

        let mut sum = 0;

        for (y, row) in input.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == Cell::Box {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    });

    let inputs = inputs.modify(|input| {
        let mut map = Vec::new();

        for y in 0..input.map.len() {
            map.push(Vec::new());

            for x in 0..input.map[y].len() {
                map[y].push(if input.map[y][x] == Cell::Box {
                    Cell::BoxLeft
                } else {
                    input.map[y][x]
                });

                match input.map[y][x] {
                    Cell::Box => map[y].push(Cell::BoxRight),
                    Cell::Nothing | Cell::Robot => map[y].push(Cell::Nothing),
                    Cell::Wall => map[y].push(Cell::Wall),
                    _ => unreachable!(),
                }
            }
        }

        Input { map, ..input }
    });

    shared::solution_fn(2, &inputs, 9021, |mut input| {
        let robot = two_dimensional_find(&input.map, Cell::Robot).expect("Cannot find robot");
        input.map[robot.1][robot.0] = Cell::Nothing;

        walk::<true>(&mut input.map, &input.moves, robot);

        let mut sum = 0;

        for (y, row) in input.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == Cell::BoxLeft {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    });
}

shared::runner!();
