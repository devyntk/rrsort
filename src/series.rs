use crate::r#match::Match;
use crate::{ALLIANCES, MATCHES_PER_SERIES, NUM_SERIES};
use itertools::Itertools;
use nalgebra::SMatrix;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type AllianceMatrix = SMatrix<usize, ALLIANCES, ALLIANCES>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Series {
    pub matches: [Match; MATCHES_PER_SERIES],
    #[serde(skip)]
    pub plays: AllianceMatrix,
}

impl Series {
    pub fn get_permutations() -> Vec<Series> {
        let mut res = Vec::new();
        for matches in (1..=ALLIANCES)
            .permutations(ALLIANCES)
            .map(|s| {
                s.chunks(2)
                    .map(|a| Match(a[0], a[1]))
                    .collect::<Vec<Match>>()
            })
            .unique()
        {
            let matches: [Match; MATCHES_PER_SERIES] = matches.try_into().unwrap();

            let mut series = Series {
                matches,
                plays: SMatrix::from_diagonal_element(0),
            };
            matches.iter().for_each(|m| series.mark_played(m));
            println!("{:?}, {:?}", series, series.get_fields(0));
            res.push(series);
        }
        println!("Total generated Series: {:?}", res.len());
        return res;
    }

    fn mark_played(&mut self, m: &Match) {
        self.plays[(m.0 - 1, m.1 - 1)] += 1;
        self.plays[(m.1 - 1, m.0 - 1)] += 1;
    }

    pub fn get_match_num(&self) -> [usize; ALLIANCES] {
        (1..=ALLIANCES)
            .map(|a| {
                for i in 0..MATCHES_PER_SERIES {
                    if self.matches[i].0 == a || self.matches[i].1 == a {
                        return i;
                    }
                }
                panic!("Can't find played match for alliance {}", a);
            })
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
    }

    pub fn get_last_teams(&self) -> [&usize; 2] {
        [
            &self.matches[MATCHES_PER_SERIES - 1].0,
            &self.matches[MATCHES_PER_SERIES - 1].1,
        ]
    }
    pub fn get_first_teams(&self) -> [&usize; 2] {
        [&self.matches[0].0, &self.matches[0].1]
    }

    pub fn get_fields(&self, series: usize) -> [isize; ALLIANCES] {
        let mut res: [isize; ALLIANCES] = [0; ALLIANCES];
        for i in 0..MATCHES_PER_SERIES {
            let modifier = if (i + series * NUM_SERIES) % 2 == 0 {
                1
            } else {
                -1
            };
            res[(self.matches[i].0) - 1] = modifier;
            res[(self.matches[i].1) - 1] = modifier;
        }
        res
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
