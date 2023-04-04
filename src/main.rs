#![feature(iter_array_chunks)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(int_roundings)]
#![feature(exclusive_range_pattern)]

mod r#match;
mod schedule;
mod series;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::schedule::Schedule;
use crate::series::Series;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use a pre-generated valid schedules file
    #[arg(short, long, value_name = "FILE")]
    schedules: Option<PathBuf>,

    /// Only validate this number of schedules during schedule generation
    #[arg(short, long)]
    take_top: Option<u64>,

    /// Force generation of new schedule
    #[arg(short, long)]
    force_new: bool
}

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

fn get_valid_schedules(take: Option<u64>) -> Vec<Schedule> {
    let all_series = Series::get_permutations();
    let permut_count = total_iterations(all_series.len() as f64);

    let count = if let Some(num) = take {
        if permut_count > num {
            num
        } else {
            permut_count
        }
    } else {
        permut_count
    };


    all_series
        .iter()
        .permutations(NUM_SERIES)
        .take(count as usize)
        .par_bridge()
        .progress_count(count)
        .filter_map(|s| Schedule::from_series(s))
        .filter(|s| s.check_back_to_back())
        .collect()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut schedules: Vec<Schedule> = if let Some(path) = cli.schedules {
        if !path.exists() || cli.force_new {
            let schedules = get_valid_schedules(cli.take_top);
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer(writer, &schedules)?;
            schedules
        } else {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        }
    } else {
        get_valid_schedules(cli.take_top)
    };

    for schedule in &schedules {
        println!("{}", schedule)
    }

    println!("Valid schedules: {:?}", &schedules.len());

    schedules.sort_unstable();
    println!("best: {}", schedules.last().unwrap());
    Ok(())
}
