use crate::game::mv::Move;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub type HeuristicOptions = [[f64; 7]; 3];

#[derive(Debug, Clone)]
pub struct RaveValue {
    visits: u128,
    total_score: i128,
    mean: f64,
}

impl RaveValue {
    #[must_use]
    pub fn new(score: i32) -> Self {
        Self {
            visits: 1,
            total_score: i128::from(score),
            mean: f64::from(score),
        }
    }

    #[must_use]
    pub fn new_with_data(visits: u128, total_score: i128) -> Self {
        Self {
            visits,
            total_score,
            mean: total_score as f64 / visits as f64,
        }
    }

    pub fn update(&mut self, score: i32) {
        self.total_score += i128::from(score);
        self.visits += 1;
        self.mean = self.total_score as f64 / self.visits as f64;
    }

    #[must_use]
    pub fn get_mean(&self) -> f64 {
        self.mean
    }

    #[must_use]
    pub fn get_total(&self) -> i128 {
        self.total_score
    }

    #[must_use]
    pub fn get_visits(&self) -> u128 {
        self.visits
    }
    pub fn uniform(&mut self, weight: u128) {
        self.total_score = (self.mean * weight as f64) as i128;
        self.visits = weight;
    }
}

impl Default for RaveValue {
    fn default() -> RaveValue {
        RaveValue {
            visits: 0,
            total_score: 0,
            mean: 0.,
        }
    }
}

type Turn = u8;

// TODO: Consider these state heuristics:
//    Count remaining specials to place
//    "Centrality". Give points to each placed piece as close they are to the center
//    Connectedness. Give points to connections
//    "Potential" connectedness. Is there a way to calculate this? Clustered frontiers?
#[derive(Clone)]
pub struct Heuristics {
    pub exploration_variables: [f64; 7],
    pub special_cost: [f64; 7],
    pub frontier_size: [f64; 7],
    pub rave: HashMap<Move, RaveValue>,
    pub local_rave: HashMap<(Turn, Move), RaveValue>,
    pub rave_jitter: f64,
    pub rave_exploration_bias: f64,
    pub use_rave: bool,
    pub tree_reuse: bool,
}

impl Heuristics {
    #[must_use]
    pub fn new(raw: HeuristicOptions) -> Self {
        Self {
            exploration_variables: raw[0], // exploration bias
            special_cost: raw[1],
            frontier_size: raw[2],
            rave: HashMap::new(),
            local_rave: HashMap::new(),
            rave_jitter: 0.5,
            rave_exploration_bias: 18.0,
            use_rave: true,
            tree_reuse: true,
        }
    }

    #[must_use]
    /// Load a `.csv`-file at the given `path` and return a new instance of Heuristics
    /// # Panics
    /// Panics if the file could not be found or if there was an error reading the file
    /// # Example file
    /// ```csv
    /// exploration_variables,special_cost,frontier_size
    /// 1.5,9.0,0.9
    /// 1.5,8.0,0.8
    /// 1.5,6.0,0.7
    /// 1.5,1.0,0.6
    /// 1.5,0.0,0.4
    /// 1.5,0.0,0.2
    /// 1.5,0.0,0.0
    /// ```
    pub fn from_csv(path: &str) -> Self {
        let mut file = File::open(path).expect("Error loading Heuristics: Could not find path");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error loading Heuristics: Could not read file to string");

        let mut heuristics = Heuristics::default();
        let mut lines = contents.split('\n');
        lines.next(); // Skip header
        for (i, line) in lines.enumerate() {
            let mut values = line.split(',');
            heuristics.exploration_variables[i] = values.next().unwrap().parse().unwrap();
            heuristics.special_cost[i] = values.next().unwrap().parse().unwrap();
            heuristics.frontier_size[i] = values.next().unwrap().parse().unwrap();
        }
        heuristics
    }

    /// Export this instance of Heuristics to a `.csv`-file at the given `path`
    /// # Errors
    /// Erros if there was an error writing to the file
    pub fn to_csv(self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"exploration_variables,special_cost,frontier_size\n")?;
        for i in 0..7 {
            file.write_all(
                format!(
                    "{},{},{}\n",
                    self.exploration_variables[i], self.special_cost[i], self.frontier_size[i]
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }

    #[must_use]
    pub fn get_rave(&self, turn: u8, mv: &Move) -> f64 {
        const MAGIC_DEFAULT_RAVE_SCORE: f64 = f64::MAX;
        match self.local_rave.get(&(turn, *mv)) {
            None => MAGIC_DEFAULT_RAVE_SCORE,
            Some(rave_value) => rave_value.get_mean(),
        }
    }

    pub fn update_rave(&mut self, turn: u8, mv: &Move, score: f64) {
        self.local_rave
            .entry((turn, *mv))
            .or_insert_with(RaveValue::default)
            .update(score as i32);
    }

    /// Export this instance of Rave to a `.csv`-file at the given `path`
    ///
    /// # Errors
    /// Returns an error if the path could not be written to
    pub fn load_rave(&mut self, path: &str) {
        let mut file = File::open(path).expect("Error loading Rave: Could not find path");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error loading Rave: Could not read file to string");

        let map: Result<HashMap<_, _, _>, ()> = contents
            .split('\n')
            .skip(1)
            .map(|row| row.split(',').collect::<Vec<_>>())
            .map(|row| {
                let place = Move::from_str(row[0])?;
                let visits = row[1].parse().expect("load_rave: could not parse visits");
                let total = row[2].parse().expect("load_rave: could not parse total");
                Ok((place, RaveValue::new_with_data(visits, total)))
            })
            .collect();
        let map = map.expect("Error loading Rave: Could not parse file to HashMap");
        self.rave = map;
    }

    /// Export this instance of Local Rave to a `.csv`-file at the given `path`
    ///
    /// # Errors
    /// Returns an error if the path could not be written to
    pub fn dump_local_rave(&self, path: &str) -> Result<(), std::io::Error> {
        let header = String::from("turn,move,visits,total");
        let data = self
            .local_rave
            .iter()
            .map(|((turn, mv), val)| format!("{turn},{mv:?},{},{}", val.visits, val.total_score))
            .fold(header, |csv, next| format!("{csv}\n{next}"));

        let mut file = File::create(path)?;
        let mut buffer = Vec::new();
        write!(&mut buffer, "{data}")?;
        file.write_all(buffer.as_slice())
    }

    pub fn load_local_rave(&mut self, path: &str) {
        let mut file = File::open(path).expect("Error loading Rave: Could not find path");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error loading Rave: Could not read file to string");

        let map: Result<HashMap<_, _, _>, ()> = contents
            .split('\n')
            .skip(1)
            .map(|row| row.split(',').collect::<Vec<_>>())
            .map(|row| {
                let turn = u8::from_str(row[0]).expect("load_local_rave: could not parse turn");
                let place = Move::from_str(row[1])?;
                let visits = row[2].parse().expect("load_rave: could not parse visits");
                let total = row[3].parse().expect("load_rave: could not parse total");
                Ok(((turn, place), RaveValue::new_with_data(visits, total)))
            })
            .collect();
        let map = map.expect("Error loading Rave: Could not parse file to HashMap");
        self.local_rave = map;
    }

    pub fn uniform_rave(&mut self, weight: u128) {
        self.rave.values_mut().for_each(|val| val.uniform(weight));
    }

    pub fn uniform_local_rave(&mut self, weight: u128) {
        self.local_rave
            .values_mut()
            .for_each(|val| val.uniform(weight));
    }

    #[must_use]
    pub fn exploration_bias(&self, turn: usize) -> f64 {
        if turn == 7 {
            1.0
        } else {
            self.exploration_variables[turn]
        }
    }
}

impl Default for Heuristics {
    fn default() -> Heuristics {
        Heuristics::new([
            [1.5; 7],
            [9.0, 8.0, 6.0, 1.0, 0.0, 0.0, 0.0],
            [0.9, 0.8, 0.7, 0.6, 0.4, 0.2, 0.0],
        ])
    }
}
