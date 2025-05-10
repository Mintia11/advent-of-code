#![feature(os_str_display)]
#![feature(path_file_prefix)]
#![feature(let_chains)]
#![feature(slice_index_methods)]
#![feature(const_for)]

use std::{fmt::Debug, time::Duration};

pub use input::*;
use internal::{CollectedData, COLLECTED_DATA};
pub use math::*;
pub use str_trait::*;

pub mod input;
pub mod internal;
pub mod math;
pub mod num_traits;
pub mod str_trait;

#[macro_export]
macro_rules! runner {
    () => {
        pub fn run() {
            $crate::internal::in_run();

            main()
        }
    };

    ($days:ident) => {
        fn main() {
            $crate::internal::main_runner($days)
        }
    };
}

pub fn solution_fn<T, F, R>(part: usize, inputs: &Input<T>, sample_solution: R, solve: F)
where
    T: Clone,
    R: PartialEq + Debug,
    F: Fn(T) -> R,
{
    let inputs = inputs.clone();

    internal::running_sample();
    let part_data = match (part, inputs.sample_part2) {
        (1, _) => inputs.sample_part1,
        (2, None) => inputs.sample_part1,
        (2, Some(i)) => i,
        _ => unreachable!(),
    };

    let sample = solve(part_data);
    if sample != sample_solution {
        panic!(
            r#"Didn't solve using the sample input
 - Expected {sample_solution:?}
 - Found {sample:?}"#
        );
    }

    internal::running_real();
    let (res, dur) = timed_fn(|| solve(inputs.real));

    let time_fn = if part == 1 {
        CollectedData::set_part1_time
    } else {
        CollectedData::set_part2_time
    };

    time_fn(&COLLECTED_DATA, dur.as_secs_f64());

    if !internal::is_running_as_single() {
        println!("Part {} took {:?}", part, dur);
        println!(" - Result: {:?}", res);
    }
}

pub fn timed_fn<T, F>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

pub fn concat<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut out = Vec::with_capacity(a.len() + b.len());

    out.extend_from_slice(a);
    out.extend_from_slice(b);

    out
}

#[must_use]
pub fn is_crlf(s: &str) -> bool {
    s.contains("\r\n")
}
