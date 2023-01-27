use crate::board::Board;
use crate::game::mv::Move;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
pub mod nn;
use nn::edge_strategy::EdgeStrategy;
mod rave;

pub type HeuristicOptions = [[f64; 7]; 3];

type Turn = u8;

// TODO: Consider these state heuristics:
// * "Centrality". Give points to each placed piece as close they are to the center
// * Connectedness. Give points to connections made, minus one for the piece placed from.
// * "Potential" connectedness. Is there a way to calculate this? Clustered frontiers?
// * Exit connection. Assign points for placements on the exit
//
#[derive(Clone)]
pub struct Heuristics {
    pub exploration_variables: [f64; 7],
    pub special_cost: [f64; 7],
    pub frontier_size: [f64; 7],
    pub rave: HashMap<Move, rave::Value>,
    pub local_rave: HashMap<(Turn, Move), rave::Value>,
    pub rave_jitter: f64,
    pub rave_exploration_bias: f64,
    pub use_rave: bool,
    pub tree_reuse: bool,
    pub use_nn: bool,
    pub move_evaluation_nn: EdgeStrategy,
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
            use_nn: true,
            move_evaluation_nn: EdgeStrategy::load(),
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

        let mut heuristics = Self::new([[0.0; 7], [0.0; 7], [0.0; 7]]);
        let mut lines = contents.split('\n');
        lines.next(); // Skip header
        for (i, line) in lines.enumerate() {
            if i > 6 {
                break;
            }
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
            .or_insert_with(rave::Value::default)
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
                Ok((place, rave::Value::new_with_data(visits, total)))
            })
            .collect();
        let map = map.expect("Error loading Rave: Could not parse file to HashMap");
        self.rave = map;
    }

    /// Export this instance of Local Rave to a `.csv`-file at the given `path`
    /// TODO: move to rave.rs
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
                Ok(((turn, place), rave::Value::new_with_data(visits, total)))
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

    #[must_use]
    pub fn get_special_cost(&self, turn: usize, mv: &Move) -> f64 {
        if let Move::Place(placement) = mv {
            if turn < 7 && 8 < placement.piece && placement.piece < 15 {
                self.special_cost[turn - 1]
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    #[must_use]
    pub fn get_nn_cost(&self, board: &Board, mv: &Move) -> f64 {
        self.move_evaluation_nn.predict(board, mv) as f64
    }
}

impl Default for Heuristics {
    fn default() -> Self {
        Self::new([
            [1.5; 7],
            [9.0, 8.0, 6.0, 1.0, 0.0, 0.0, 0.0],
            [0.9, 0.8, 0.7, 0.6, 0.4, 0.2, 0.0],
        ])
    }
}
