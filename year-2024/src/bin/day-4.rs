use shared::{const_rotate_matrix, two_dimensional_get};

fn main() {
    let inputs = shared::parse_input(|i| {
        i.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 18, |input| {
        let mut sum = 0;

        for (y, line) in input.iter().enumerate() {
            for (x, &c) in line.iter().enumerate() {
                if c == 'X' {
                    let mut is_xmas = [true; 8];
                    for d in 0..4 {
                        const XMAS: &[char] = &['X', 'M', 'A', 'S'];

                        let directions: [(isize, isize); 8] = [
                            (d, 0),   // Right
                            (-d, 0),  // Left
                            (0, d),   // Bottom
                            (0, -d),  // Top
                            (d, d),   // Bottom Right
                            (-d, d),  // Bottom Left
                            (d, -d),  // Top Right
                            (-d, -d), // Top Left
                        ];

                        'next_direction: for (i, dir) in directions.iter().enumerate() {
                            let chr = two_dimensional_get(
                                &input,
                                x.wrapping_add_signed(dir.0),
                                y.wrapping_add_signed(dir.1),
                            );

                            if let Some(chr) = chr {
                                if chr != XMAS[d.unsigned_abs()] {
                                    is_xmas[i] = false;
                                    continue 'next_direction;
                                }
                            } else {
                                is_xmas[i] = false;

                                continue 'next_direction;
                            }
                        }
                    }

                    for x in is_xmas {
                        if x {
                            sum += 1;
                        }
                    }
                }
            }
        }

        sum
    });

    shared::solution_fn(2, &inputs, 9, |input| {
        const ROT0_MATRIX: [[Option<char>; 3]; 3] = [
            [Some('M'), None, Some('S')],
            [None, Some('A'), None],
            [Some('M'), None, Some('S')],
        ];
        const ROT1_MATRIX: [[Option<char>; 3]; 3] = const_rotate_matrix(&ROT0_MATRIX, None);
        const ROT2_MATRIX: [[Option<char>; 3]; 3] = const_rotate_matrix(&ROT1_MATRIX, None);
        const ROT3_MATRIX: [[Option<char>; 3]; 3] = const_rotate_matrix(&ROT2_MATRIX, None);

        const PATTERNS: [[[Option<char>; 3]; 3]; 4] =
            [ROT0_MATRIX, ROT1_MATRIX, ROT2_MATRIX, ROT3_MATRIX];

        let mut sum = 0;

        for (y, line) in input.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                let mut got = [[None; 3]; 3];

                for (y1, row) in got.iter_mut().enumerate() {
                    for (x1, col) in row.iter_mut().enumerate() {
                        *col = two_dimensional_get(&input, x + x1, y + y1);
                    }
                }

                let mut is_x_mas = [true; 4];
                'next_pattern: for (i, pattern) in PATTERNS.iter().enumerate() {
                    for (y1, row) in got.iter().enumerate() {
                        for (x1, &col) in row.iter().enumerate() {
                            let matching = match (col, pattern[y1][x1]) {
                                (_, None) => true,
                                (None, Some(_)) => false,
                                (Some(a), Some(b)) => a == b,
                            };

                            if !matching {
                                is_x_mas[i] = false;
                                continue 'next_pattern;
                            }
                        }
                    }
                }

                for x in is_x_mas {
                    if x {
                        sum += 1;
                    }
                }
            }
        }

        sum
    });
}

shared::runner!();
