use hashbrown::HashSet;
use nalgebra::Vector2;
use regex::Regex;
use shared::internal::is_running_sample;

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Vector2<isize>,
    velocity: Vector2<isize>,
}

fn walk(robot: &mut Robot, max_x: isize, max_y: isize) {
    robot.position.x = (robot.position.x + robot.velocity.x).rem_euclid(max_x);
    robot.position.y = (robot.position.y + robot.velocity.y).rem_euclid(max_y);
}

fn count_in_formation(positions: &HashSet<Vector2<isize>>) -> usize {
    const DELTAS: [Vector2<isize>; 4] = [
        Vector2::new(1, 0),
        Vector2::new(0, -1),
        Vector2::new(-1, 0),
        Vector2::new(0, 1),
    ];

    let mut t = 0;

    for pos in positions {
        for delta in DELTAS {
            if positions.contains(&(pos + delta)) {
                t += 1;
            }
        }
    }

    t
}

fn main() {
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let inputs = shared::parse_input(|s| {
        let mut out = Vec::new();

        for line in s.lines() {
            let captures = regex.captures(line).unwrap();

            let pos_x = shared::math::int(&captures[1]);
            let pos_y = shared::math::int(&captures[2]);
            let vel_x = shared::math::int(&captures[3]);
            let vel_y = shared::math::int(&captures[4]);

            out.push(Robot {
                position: Vector2::new(pos_x, pos_y),
                velocity: Vector2::new(vel_x, vel_y),
            })
        }

        out
    });

    shared::solution_fn(1, &inputs, 12, |mut input| {
        let max_x = if is_running_sample() { 11 } else { 101 };
        let max_y = if is_running_sample() { 7 } else { 103 };

        for _ in 0..100 {
            input.iter_mut().for_each(|r| walk(r, max_x, max_y));
        }

        let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);

        for &Robot { position: pos, .. } in &input {
            if pos.x < max_x / 2 && pos.y < max_y / 2 {
                c1 += 1;
            } else if pos.x > max_x / 2 && pos.y < max_y / 2 {
                c2 += 1;
            } else if pos.x < max_x / 2 && pos.y > max_y / 2 {
                c3 += 1;
            } else if pos.x > max_x / 2 && pos.y > max_y / 2 {
                c4 += 1;
            }
        }

        c1 * c2 * c3 * c4
    });

    shared::solution_fn(2, &inputs, 0, |mut input| {
        if is_running_sample() {
            return 0;
        }

        let max_x = if is_running_sample() { 11 } else { 101 };
        let max_y = if is_running_sample() { 7 } else { 103 };

        let mut secs = 0;

        loop {
            input.iter_mut().for_each(|r| walk(r, max_x, max_y));
            secs += 1;

            let positions: HashSet<Vector2<isize>> = input
                .iter()
                .map(|&Robot { position, .. }| position)
                .collect();

            if count_in_formation(&positions) >= 1000 {
                break;
            }
        }

        secs
    });
}

shared::runner!();
