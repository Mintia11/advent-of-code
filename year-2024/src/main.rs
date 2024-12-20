#![feature(let_chains)]

shared::runner!(DAYS);

#[rustfmt::skip]
const DAYS: &[fn()] = &[day_1::run, day_2::run, day_3::run, day_4::run, day_5::run, day_6::run, day_7::run, day_8::run, day_9::run, day_10::run, day_11::run, day_12::run, day_13::run, day_14::run];

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

#[path = "bin/day-6.rs"]
mod day_6;

#[path = "bin/day-7.rs"]
mod day_7;

#[path = "bin/day-8.rs"]
mod day_8;

#[path = "bin/day-9.rs"]
mod day_9;

#[path = "bin/day-10.rs"]
mod day_10;

#[path = "bin/day-11.rs"]
mod day_11;

#[path = "bin/day-12.rs"]
mod day_12;

#[path = "bin/day-13.rs"]
mod day_13;

#[path = "bin/day-14.rs"]
mod day_14;
