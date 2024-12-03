use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static IS_RUNNING_AS_SINGLE: AtomicBool = AtomicBool::new(false);
static CURRENT_DAY: AtomicUsize = AtomicUsize::new(0);
static IS_RUNNING_SAMPLE: AtomicBool = AtomicBool::new(false);

pub(crate) fn is_running_as_single() -> bool {
    IS_RUNNING_AS_SINGLE.load(Ordering::Relaxed)
}

pub(crate) fn current_day() -> usize {
    CURRENT_DAY.load(Ordering::Relaxed)
}

pub(crate) fn running_sample() {
    IS_RUNNING_SAMPLE.store(true, Ordering::Relaxed);
}

pub(crate) fn running_real() {
    IS_RUNNING_SAMPLE.store(false, Ordering::Relaxed);
}

pub fn is_running_sample() -> bool {
    IS_RUNNING_SAMPLE.load(Ordering::Relaxed)
}

pub fn in_run() {
    IS_RUNNING_AS_SINGLE.store(true, Ordering::Relaxed);
    CURRENT_DAY.fetch_add(1, Ordering::Relaxed);

    println!("Day {}:", current_day());
}
