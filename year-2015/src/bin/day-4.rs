fn mine(secret_key: &str, prefix: &str) -> u64 {
    let mut number: u64 = 1;
    loop {
        let input = format!("{}{}", secret_key, number);
        let hash = format!("{:x}", md5::compute(input));
        if hash.starts_with(prefix) {
            return number;
        }
        number += 1;
    }
}

fn main() {
    let inputs = shared::parse_input(|s| s.to_string());

    shared::solution_fn(1, &inputs, 1300852, |input| mine(&input, "00000"));
    shared::solution_fn(2, &inputs, 27989252, |input| mine(&input, "000000"));
}

shared::runner!();
