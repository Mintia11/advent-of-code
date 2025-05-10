#![allow(incomplete_features)]
#![feature(let_chains)]
#![feature(adt_const_params)]
#![feature(portable_simd)]

shared::runner!(DAYS);

#[rustfmt::skip]
const DAYS: &[fn()] = &[day_1::run, day_2::run, day_3::run, day_4::run, day_5::run, day_6::run, day_7::run, day_8::run, day_9::run, day_10::run, day_11::run, day_12::run, day_13::run, day_14::run, day_15::run, day_16::run, day_17::run, day_18::run, day_19::run, day_20::run, day_21::run, day_22::run, day_23::run, day_24::run, day_25::run];

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

#[path = "bin/day-15.rs"]
mod day_15;

#[path = "bin/day-16.rs"]
mod day_16;

#[path = "bin/day-17.rs"]
mod day_17;

#[path = "bin/day-18.rs"]
mod day_18;

#[path = "bin/day-19.rs"]
mod day_19;

#[path = "bin/day-20.rs"]
mod day_20;

#[path = "bin/day-21.rs"]
mod day_21;

#[path = "bin/day-22.rs"]
mod day_22;

#[path = "bin/day-23.rs"]
mod day_23;

#[path = "bin/day-24.rs"]
mod day_24;

#[path = "bin/day-25.rs"]
mod day_25;
