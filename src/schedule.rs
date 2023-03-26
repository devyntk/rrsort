use crate::series::{AllianceMatrix, Series};
use crate::{ALLIANCES, NUM_SERIES};
use nalgebra::one;
use std::fmt;

#[derive(Debug)]
pub struct Schedule<'a> {
    series: [&'a Series; NUM_SERIES],
}

fn is_valid(entry: &Vec<&Series>) -> bool {
    let mut check: AllianceMatrix = one();
    for series in entry {
        check += series.plays;
    }
    check == AllianceMatrix::repeat(1)
}

impl Schedule<'_> {
    pub fn check_back_to_back(&self) -> bool {
        for int in 0..NUM_SERIES - 1 {
            for team in self.series[int].get_last_teams() {
                if self.series[int + 1].get_first_teams().contains(&team) {
                    return false;
                }
            }
        }
        true
    }

    pub fn from_series(entry: Vec<&Series>) -> Option<Schedule> {
        if is_valid(&entry) {
            Some(Schedule {
                series: entry.try_into().unwrap(),
            })
        } else {
            None
        }
    }

    pub fn avg_min_delta(&self) -> f64 {
        let series_plays: Vec<[usize; ALLIANCES]> =
            self.series.iter().map(|s| s.get_match_num()).collect();

        (0..ALLIANCES)
            .into_iter()
            .map(|a| {
                (0..NUM_SERIES - 1)
                    .into_iter()
                    .map(|i| (3 + series_plays[i + 1][a] - series_plays[i][a]) as f64)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .sum::<f64>()
            / (ALLIANCES as f64)
    }

    pub fn max_field_sep(&self) -> f64 {
        let fields: Vec<[isize; ALLIANCES]> = (0..NUM_SERIES)
            .into_iter()
            .map(|i| self.series[i].get_fields(i))
            .collect();

        (0..ALLIANCES)
            .into_iter()
            .map(|a| fields.iter().map(|i| i[a] as f64).sum::<f64>() / NUM_SERIES as f64)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}

impl fmt::Display for Schedule<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "red | blue | (avg min turnaround {}, max field sep {}",
            self.avg_min_delta(),
            self.max_field_sep()
        )?;
        for s in self.series {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}