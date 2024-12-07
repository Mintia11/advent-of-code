#![feature(os_str_display)]
#![feature(path_file_prefix)]
#![feature(let_chains)]
#![feature(slice_index_methods)]
#![feature(const_for)]

use std::{fmt::Debug, time::Duration};

pub use input::*;
pub use math::*;

pub mod input;
pub mod internal;
pub mod math;

#[macro_export]
macro_rules! runner {
    () => {
        pub fn run() {
            $crate::internal::in_run();

            main()
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
    if let Some(part2) = inputs.sample_part2
        && part == 2
    {
        let sample = solve(part2);
        if sample != sample_solution {
            panic!(
                r#"Didn't solve using the sample input
 - Expected {sample_solution:?}
 - Found {sample:?}"#
            );
        }
    } else {
        let sample = solve(inputs.sample_part1);
        if sample != sample_solution {
            panic!(
                r#"Didn't solve using the sample input
 - Expected {sample_solution:?}
 - Found {sample:?}"#
            );
        }
    }

    internal::running_real();
    let (res, dur) = timed_fn(|| solve(inputs.real));

    println!(
        "{}Part {} took {:?}",
        if internal::is_running_as_single() {
            "  "
        } else {
            ""
        },
        part,
        dur
    );
    println!(
        "{} - Result: {:?}",
        if internal::is_running_as_single() {
            "  "
        } else {
            ""
        },
        res
    );
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
