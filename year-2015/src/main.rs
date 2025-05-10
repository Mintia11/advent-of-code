shared::runner!(DAYS);

#[rustfmt::skip]
const DAYS: &[fn()] = &[day_1::run, day_2::run, day_3::run, day_4::run, day_4::run, day_5::run];

#[path = "bin/day-1.rs"]
mod day_1;

#[path = "bin/day-2.rs"]
mod day_2;

#[path = "bin/day-3.rs"]
mod day_3;

#[path = "bin/day-4.rs"]
mod day_4;

#[path = "bin/day-5.rs"]
mod day_5;
