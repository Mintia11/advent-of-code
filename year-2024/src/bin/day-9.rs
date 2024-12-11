use std::collections::BTreeSet;

use shared::uint_char;

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut file = 0;

        s.chars()
            .map(uint_char)
            .enumerate()
            .flat_map(|(i, chr)| {
                if i % 2 != 0 {
                    vec![None; chr]
                } else {
                    let f = file;
                    file += 1;

                    vec![Some(f); chr]
                }
            })
            .collect::<Vec<_>>()
    });

    shared::solution_fn(1, &inputs, 1928, |mut input| {
        let number_of_files = input
            .iter()
            .fold(0, |add, i| if let Some(_) = i { add + 1 } else { add });

        let mut free_indicies = input[..number_of_files]
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.is_none().then_some(i))
            .collect::<Vec<_>>();

        free_indicies.reverse();

        while let Some(new) = free_indicies.pop() {
            let to_swap = input
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, e)| e.is_some().then_some(i))
                .unwrap();

            input.swap(to_swap, new);
        }

        input
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.map(|e| i * e))
            .sum()
    });

    shared::solution_fn(2, &inputs, 2858, |mut input| {
        let mut free_blocks: BTreeSet<(usize, usize)> = BTreeSet::new(); // Off, Size

        {
            let (mut start, mut length) = (0, 0);
            let mut is_new_block = true;
            for i in 0..input.len() {
                if input[i].is_none() && i != input.len() - 1 {
                    length += 1;
                    if is_new_block {
                        start = i;
                        is_new_block = false;
                    }
                } else if length > 0 {
                    free_blocks.insert((start, length));
                    length = 0;
                    is_new_block = true;
                }
            }
        }

        let mut file_blocks: Vec<(Option<usize>, usize, usize)> =
            Vec::with_capacity(input.len() / 2);

        {
            let (mut start, mut prev_id) = (0, None);
            for i in 0..input.len() {
                let cur_id = input[i];
                if cur_id != prev_id {
                    if prev_id.is_some() {
                        file_blocks.push((prev_id, start, i - start));
                    }

                    if cur_id.is_some() {
                        start = i;
                    }
                }

                prev_id = cur_id;
            }

            if prev_id.is_some() {
                file_blocks.push((prev_id, start, input.len() - start));
            }
        }

        for file in file_blocks.iter().rev() {
            let free_block = free_blocks
                .iter()
                .find(|block| block.0 < file.1 && block.1 >= file.2);

            if free_block.is_none() {
                continue;
            }

            let free_block = free_block.unwrap().clone();

            for i in 0..file.2 {
                input[free_block.0 + i] = file.0;
                input[file.1 + i] = None;
            }

            if free_block.1 > file.2 {
                let new_start = free_block.0 + file.2;
                let new_size = free_block.1 - file.2;

                free_blocks.insert((new_start, new_size));
            }

            free_blocks.remove(&free_block);
        }

        input
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.map(|e| i * e))
            .sum()
    });
}

shared::runner!();
