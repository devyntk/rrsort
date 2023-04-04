#![feature(iter_array_chunks)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(int_roundings)]
#![feature(exclusive_range_pattern)]

mod r#match;
mod schedule;
mod series;

use crate::schedule::Schedule;
use crate::series::Series;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;

const ALLIANCES: usize = 6;

const NUM_SERIES: usize = ALLIANCES - 1;
const MATCHES_PER_SERIES: usize = ALLIANCES.div_floor(2);

const fn factorial(n: f64) -> f64 {
    match n {
        x if x < 1.1 => 1.0,
        n => factorial(n - 1.0) * n,
    }
}

const fn total_iterations(series: f64) -> u64 {
    (factorial(series) / factorial(series - NUM_SERIES as f64)) as u64
}
const TAKE_TOP: u64 = 10_000_000;

fn main() {
    let all_series = Series::get_permutations();
    let permut_count = total_iterations(all_series.len() as f64);
    let count = if permut_count > TAKE_TOP {
        TAKE_TOP
    } else {
        permut_count
    };

    let mut schedules: Vec<Schedule> = all_series
        .iter()
        .permutations(NUM_SERIES)
        .take(count as usize)
        .par_bridge()
        .progress_count(count)
        .filter_map(|s| Schedule::from_series(s))
        .filter(|s| s.check_back_to_back())
        .collect();

    for schedule in &schedules {
        println!("{}", schedule)
    }

    println!("Valid schedules: {:?}", &schedules.len());

    schedules.sort_unstable();
    println!("best: {}", schedules.last().unwrap())
}
