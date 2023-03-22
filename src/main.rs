use std::fmt;
use itertools::Itertools;
use nalgebra::{one, SMatrix};
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

const ALLIANCES: usize = 6;

const NUM_SERIES: usize = ALLIANCES - 1;
const MATCHES_PER_SERIES: usize = ALLIANCES / 2;

fn main() {
    let all_series = Series::get_permutations();
    let schedules: Vec<Schedule> = all_series.iter()
        .permutations(NUM_SERIES)
        .par_bridge()
        .map(|s| Schedule{series: s.try_into().unwrap()})
        .filter(|s| s.is_valid())
        .filter(|s| s.check_back_to_back())
        .collect();

    for schedule in &schedules {
        println!("{}", schedule)
    }

    println!("Valid schedules: {:?}", &schedules.len())
}

#[derive(Debug)]
struct Schedule<'a> {
    series: [&'a Series; NUM_SERIES]
}

impl Schedule<'_> {
    fn is_valid(&self) -> bool {
        let mut check: AllianceMatrix = one();
        for series in &self.series {
            check += series.plays;
        }
        check == AllianceMatrix::repeat(1)
    }

    fn check_back_to_back(&self) -> bool {
        for int in 0..NUM_SERIES {
            for team in self.series[int].get_last_teams(){
                if self.series[int+1].get_first_teams().contains(&team){
                    return false;
                }
            }
        };
        true
    }
}

type AllianceMatrix = SMatrix<usize, ALLIANCES, ALLIANCES>;

#[derive(Debug)]
struct Series {
    matches: [(usize, usize); MATCHES_PER_SERIES],
    plays: AllianceMatrix
}

impl Series {
    fn get_permutations() -> Vec<Series> {
        let mut res = Vec::new();
        for teams in (1..ALLIANCES+1).permutations(ALLIANCES) {
            let matches = teams.chunks(2)
                .map(|a| (a[0],a[1]))
                .collect::<Vec<(usize,usize)>>().try_into().unwrap();
            let mut series = Series{
                matches,
                plays: SMatrix::from_diagonal_element(0)
            };
            matches.iter().for_each(|a| series.mark_played(a.0,a.1));
            println!("{:?}", series);
            res.push(series);
        }
        return res;
    }

    fn mark_played(&mut self, team1: usize, team2: usize) {
        self.plays[(team1-1, team2-1)] += 1;
        self.plays[(team2-1, team1-1)] += 1;
    }

    fn get_last_teams(&self) -> [usize; 2] {
        let last = self.matches.last().unwrap();
        [last.0, last.1]
    }
    fn get_first_teams(&self) -> [usize; 2] {
        let first = self.matches.first().unwrap();
        [first.0, first.1]
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for m in self.matches {
            write!(f, "\n {}  |  {}", m.0, m.1)?;
        }
        write!(f, " |")
    }
}
impl fmt::Display for Schedule<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "red | blue")?;
        for s in self.series {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}