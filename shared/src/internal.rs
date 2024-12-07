use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

static IS_RUNNING_AS_SINGLE: AtomicBool = AtomicBool::new(false);
static CURRENT_DAY: AtomicUsize = AtomicUsize::new(0);
static IS_RUNNING_SAMPLE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub(crate) struct CollectedData {
    parse_time: AtomicU64,
    part1_time: AtomicU64,
    part2_time: AtomicU64,
}

impl CollectedData {
    pub fn set_parse_time(&self, time: f64) {
        self.parse_time
            .store(u64::from_le_bytes(time.to_le_bytes()), Ordering::Relaxed);
    }

    pub fn parse_time(&self) -> f64 {
        f64::from_le_bytes(self.parse_time.load(Ordering::Relaxed).to_le_bytes())
    }

    pub fn set_part1_time(&self, time: f64) {
        self.part1_time
            .store(u64::from_le_bytes(time.to_le_bytes()), Ordering::Relaxed);
    }

    pub fn part1_time(&self) -> f64 {
        f64::from_le_bytes(self.part1_time.load(Ordering::Relaxed).to_le_bytes())
    }

    pub fn set_part2_time(&self, time: f64) {
        self.part2_time
            .store(u64::from_le_bytes(time.to_le_bytes()), Ordering::Relaxed);
    }

    pub fn part2_time(&self) -> f64 {
        f64::from_le_bytes(self.part2_time.load(Ordering::Relaxed).to_le_bytes())
    }
}

impl Clone for CollectedData {
    fn clone(&self) -> Self {
        Self {
            parse_time: AtomicU64::new(self.parse_time.load(Ordering::Relaxed)),
            part1_time: AtomicU64::new(self.part1_time.load(Ordering::Relaxed)),
            part2_time: AtomicU64::new(self.part2_time.load(Ordering::Relaxed)),
        }
    }
}

pub(crate) static COLLECTED_DATA: CollectedData = CollectedData {
    parse_time: AtomicU64::new(0),
    part1_time: AtomicU64::new(0),
    part2_time: AtomicU64::new(0),
};

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
}

#[cfg(feature = "runner")]
pub fn main_runner(days: &'static [fn()]) {
    use std::time::Duration;

    use indicatif::ProgressBar;
    use tabled::{
        builder::Builder,
        grid::config::Border,
        settings::{
            object::{Cell, Rows},
            span::{ColumnSpan, RowSpan},
            Alignment, Color, Style,
        },
    };

    use crate::year_bin_name;

    let mut datas: Vec<CollectedData> = Vec::with_capacity(days.len());
    let progress_bar = ProgressBar::new(days.len() as _);

    for day in days {
        day();

        progress_bar.inc(1);
        datas.push(COLLECTED_DATA.clone());
    }

    progress_bar.finish_and_clear();

    let mut table = Builder::default();

    let (year, _) = year_bin_name();
    let year = year.replace("year-", "");

    let aoc_header = format!(r"Advent of code {year}");

    table.push_record([aoc_header]);

    table.push_record(["Day", "Timings"]);
    table.push_record(["", "Total", "Parse", "Part 1", "Part 2"]);

    for (day, data) in datas.iter().enumerate() {
        let mut record = Vec::new();

        record.push(format!("{}", day + 1));

        let parse_time = Duration::from_secs_f64(data.parse_time());
        let part1_time = Duration::from_secs_f64(data.part1_time());
        let part2_time = Duration::from_secs_f64(data.part2_time());

        let total_time = parse_time + part1_time + part2_time;

        record.push(format!("{:?}", total_time));
        record.push(format!("{:?}", parse_time));
        record.push(format!("{:?}", part1_time));
        record.push(format!("{:?}", part2_time));

        table.push_record(record);
    }

    let mut table = table.build();

    let mut border_aoc_top = Border::empty();
    border_aoc_top.right_top_corner = Some('─');
    border_aoc_top.right_bottom_corner = Some('┬');

    let mut border_day = Border::empty();
    border_day.left_bottom_corner = Some('│');
    border_day.right_bottom_corner = Some('├');

    let table = table
        .with(Style::modern())
        .with(Color::BOLD)
        .modify(Rows::single(0), ColumnSpan::max())
        .modify(Cell::new(0, 0), Alignment::center())
        .modify(Cell::new(1, 0), RowSpan::new(2))
        .modify(Cell::new(1, 0), Alignment::center_vertical())
        .modify(Cell::new(1, 0), border_day)
        .modify(Cell::new(1, 1), ColumnSpan::new(4))
        .modify(Cell::new(1, 1), Alignment::center());

    for cell in 0..4 {
        table.modify(Cell::new(0, cell), border_aoc_top);
    }

    for cell in 1..4 {
        table.modify(Cell::new(1, cell), border_aoc_top);
    }

    eprintln!("{table}");
}
