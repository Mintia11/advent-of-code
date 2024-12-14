use std::{env, fs};

use crate::{
    internal::{current_day, is_running_as_single, COLLECTED_DATA},
    timed_fn,
};

pub trait InputType {
    fn path() -> &'static str;
}

pub struct Sample;

impl InputType for Sample {
    fn path() -> &'static str {
        "test-input"
    }
}

pub struct Real;

impl InputType for Real {
    fn path() -> &'static str {
        "input"
    }
}

#[derive(Clone)]
pub struct Input<T> {
    pub(crate) sample_part1: T,
    pub(crate) sample_part2: Option<T>,
    pub(crate) real: T,
}

pub(crate) fn year_bin_name() -> (String, String) {
    if is_running_as_single() {
        let exe = env::current_exe().unwrap();
        let file_name = exe.file_prefix().unwrap();
        let file_name = file_name.to_string_lossy();

        let year = file_name.to_string();
        let day = format!("day-{}", current_day());

        (year, day)
    } else {
        let exe = env::current_exe().unwrap();
        let file_name = exe.file_prefix().unwrap();
        let file_name = file_name.to_string_lossy();

        let (year, day) = file_name.split_once('_').unwrap();

        (year.to_string(), day.to_string())
    }
}

#[must_use]
pub fn parse_input<T, F>(func: F) -> Input<T>
where
    F: Fn(&str) -> T,
{
    let sample_two_parts = has_two_parts::<Sample>();
    let sample_input = sample_two_parts
        .then(|| read_input::<Sample>(Some(1)))
        .unwrap_or_else(|| read_input::<Sample>(None));
    let sample_input_part2 = sample_two_parts.then(|| read_input::<Sample>(Some(2)));
    let input = read_input::<Real>(None);

    let sample = func(&sample_input);
    let sample_part2 = sample_input_part2.map(|i| func(&i));
    let (real, dur) = timed_fn(|| func(&input));

    COLLECTED_DATA.set_parse_time(dur.as_secs_f64());

    if !is_running_as_single() {
        println!("Parsing took {:?}", dur);
    }

    Input {
        sample_part1: sample,
        sample_part2,
        real,
    }
}

#[must_use]
pub fn read_input<T: InputType>(part: Option<usize>) -> String {
    let (year, bin_name) = year_bin_name();
    let path = if let Some(part) = part {
        format!("{}/{}/{}-part{}.txt", year, T::path(), bin_name, part)
    } else {
        format!("{}/{}/{}.txt", year, T::path(), bin_name)
    };

    fs::read_to_string(path).unwrap()
}

pub fn has_two_parts<T: InputType>() -> bool {
    let (year, bin_name) = year_bin_name();

    let part1 = format!("{}/{}/{}-part1.txt", year, T::path(), bin_name);
    let part2 = format!("{}/{}/{}-part2.txt", year, T::path(), bin_name);

    matches!(
        (fs::exists(&part1), fs::exists(&part2)),
        (Ok(true), Ok(true))
    )
}
