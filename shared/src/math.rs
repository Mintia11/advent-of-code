pub fn uint(s: &str) -> usize {
    s.parse().unwrap()
}

pub fn dist(a: usize, b: usize) -> usize {
    ((a as isize) - (b as isize)).unsigned_abs()
}
