use hashbrown::HashSet;
use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Add<Direction> for (isize, isize) {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;

        match rhs {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

impl TryFrom<char> for Direction {
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

fn main() {
    let inputs = shared::parse_input(|s| {
        s.chars()
            .map(TryInto::<Direction>::try_into)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    });

    shared::solution_fn(1, &inputs, 2, |input| {
        let mut cur_pos = (0, 0);
        let mut visited = HashSet::new();

        visited.insert(cur_pos);

        for dir in input {
            cur_pos = cur_pos + dir;
            visited.insert(cur_pos);
        }

        visited.len()
    });

    shared::solution_fn(2, &inputs, 11, |input| {
        let mut santa = (0, 0);
        let mut robo_santa = (0, 0);

        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for dirs in input.chunks_exact(2) {
            let &[dir1, dir2, ..] = dirs else {
                unreachable!()
            };

            santa = santa + dir1;
            robo_santa = robo_santa + dir2;

            visited.insert(santa);
            visited.insert(robo_santa);
        }

        visited.len()
    });
}

shared::runner!();
