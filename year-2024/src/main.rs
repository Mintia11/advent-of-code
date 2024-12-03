fn main() {
    DAYS.into_iter().for_each(|f| f());
}

#[rustfmt::skip]
const DAYS: &[fn()] = &[day_1::run, day_2::run, day_3::run];

#[path = "bin/day-1.rs"]
mod day_1;

#[path = "bin/day-2.rs"]
mod day_2;

#[path = "bin/day-3.rs"]
mod day_3;
