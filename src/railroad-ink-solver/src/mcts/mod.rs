use ord_subset::OrdSubsetIterExt;

use crate::board::placement::Placement;
use crate::game::{mv::Move, roll::Roll, Game};
use std::convert::TryInto;
pub mod heuristics;
pub mod trainer;
use heuristics::Heuristics;

use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::collections::HashMap;

use crate::identity_hasher::BuildHasher;

pub type Score = f64;

#[derive(Debug, Serialize, Default)]
pub struct Node {
    pub visits: u64,
    pub total_score: f64,
    pub is_terminal: bool,
    pub heuristic: f64,
    pub children: Box<[Edge]>,
}

impl Node {
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: Box::new([]),
            is_terminal: false,
            total_score: 0.,
            heuristic: 0.,
            visits: 0,
        }
    }

    /// Expand the list of children to this node, but don't visit
    pub fn generate_children(&mut self, game: &Game) {
        self.children = game
            .generate_moves()
            .into_iter()
            .map(Edge::new)
            .collect::<Vec<Edge>>()
            .into_boxed_slice();
    }
}

#[derive(Debug)]
pub enum SingleOrMultiple {
    Single(Node),
    Multiple(HashMap<Roll, Node, BuildHasher>),
}
use SingleOrMultiple::{Multiple, Single};

impl Serialize for SingleOrMultiple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Single(node) => node.serialize(serializer),
            Multiple(nodes) => {
                let mut map = serializer.serialize_map(Some(nodes.len()))?;
                for (k, v) in nodes {
                    map.serialize_entry(&k.to_string(), v)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Edge {
    pub mv: Move,
    pub visits: u64,
    pub mean_score: Score,
    pub child: Option<SingleOrMultiple>,
}

impl Edge {
    #[must_use]
    pub fn new(mv: Move) -> Self {
        Self {
            mv,
            child: None,
            visits: 0,
            mean_score: 0.,
        }
    }

    /// One iteration of mcts
    /// Recursively `select`s through the tree,
    /// updating the `visits` count and scores along the way
    /// Rolls are simulated every time `Move::Roll` is selected
    ///
    /// # Panics
    /// Panics if no legal moves could be selected from game position
    pub fn select(&mut self, mut game: Game, heuristics: &mut Heuristics) -> Score {
        // Expand and rollout
        if self.visits == 0 {
            return self.expand(game, heuristics);
        }
        let mut generate_children = self.visits == 1 || game.turn == 0;

        let node = self.child.as_mut().unwrap();
        let node = match node {
            Multiple(nodes) => {
                debug_assert_eq!(self.mv, Move::Roll);
                // If edge Move is `Roll`, we don't get to choose which roll
                // to search. We have to actually roll the dice.
                let roll = Game::generate_roll();
                if let Some(child) = nodes.get_mut(&roll) {
                    child
                } else {
                    let child = Node::new();
                    nodes.insert(roll, child);
                    generate_children = true;
                    nodes.get_mut(&roll).unwrap()
                }
            }
            Single(node) => node,
        };
        if generate_children {
            node.generate_children(&game);
        }
        if node.is_terminal {
            // Increment `visits`. But don't change `self.mean`: it's the same, still
            self.visits += 1;
            node.visits += 1;
            node.total_score += self.mean_score;
            return self.mean_score;
        }

        assert_ne!(node.children.len(), 0, "No legal moves!");

        let mut best_exploration_value = Score::MIN;
        let mut best_child_node_index = 0;
        for (i, edge) in node.children.iter().enumerate() {
            let child_exploration_value = edge.exploration_value(self.visits, heuristics, &game);
            if child_exploration_value >= best_exploration_value {
                best_child_node_index = i;
                best_exploration_value = child_exploration_value;
            }
        }

        let child_edge = node.children.get_mut(best_child_node_index).unwrap();
        game.do_move(child_edge.mv);
        let turn = game.turn;
        let result = child_edge.select(game, heuristics);

        // Backpropagate
        self.visits += 1;
        node.visits += 1;
        node.total_score += result;
        self.mean_score = node.total_score / self.visits as f64;
        if !matches!(self.mv, Move::SetRoll(..)) {
            heuristics.update_rave(turn, &self.mv, result);
        }
        result
    }

    fn exploration_value(&self, parent_visits: u64, heuristics: &Heuristics, game: &Game) -> Score {
        let turn = game.turn;

        let ucb = self.mean_score;
        let exploration_bias = heuristics.exploration_bias(turn as usize);
        let exploration = if self.visits == 0 {
            Score::MAX
        } else {
            Score::sqrt(Score::ln(parent_visits as f64 / self.visits as f64))
        };

        let mut exploration_term = exploration_bias * exploration;

        if let Move::Place(placement) = self.mv {
            if turn < 7 && 8 < placement.piece && placement.piece < 15 {
                exploration_term -= heuristics.special_cost[turn as usize];
            }
        }

        if heuristics.use_rave {
            let k = 1.;
            let rave = heuristics.get_rave(turn, &self.mv);
            let rave = rave + heuristics.rave_exploration_bias;
            let n = self.visits as f64;
            let beta = (k / (3. * n + k)).sqrt();
            let q = (1.0 - beta) * ucb + beta * rave;

            q + exploration_term
        } else {
            ucb + exploration_term
        }
    }

    fn expand(&mut self, game: Game, heuristics: &mut Heuristics) -> Score {
        debug_assert!(self.child.is_none());

        if self.mv == Move::Roll {
            self.visits = 1;
            let nodes: HashMap<Roll, Node, BuildHasher> = HashMap::with_hasher(BuildHasher);
            self.child = Some(Multiple(nodes));
            let (score, _) = Self::rollout(game, heuristics, 0);
            score
        } else {
            let mut child = Node::new();
            let (score, is_terminal) = Self::rollout(game, heuristics, 0);
            self.visits = 1;
            self.mean_score = score;
            child.total_score = score;
            child.is_terminal = is_terminal;
            self.child = Some(Single(child));
            score
        }
    }

    /// Does random moves until `game.ended`
    /// Returns `(score, depth_zero_is_terminal)`
    fn rollout(mut game: Game, heuristics: &mut Heuristics, depth: u16) -> (Score, bool) {
        if game.ended {
            return (f64::from(game.board.score()), depth == 0);
        }

        let mut rng = rand::thread_rng();
        let moves = game.generate_moves();

        let mv = if heuristics.use_rave {
            let mv_iter = moves.into_iter();
            if heuristics.rave_jitter == 0. {
                mv_iter.ord_subset_max_by_key(|mv| heuristics.get_rave(game.turn, mv))
            } else {
                let jitter = heuristics.rave_jitter;
                mv_iter.ord_subset_max_by_key(|mv| {
                    heuristics.get_rave(game.turn, mv) + rng.gen_range(-jitter..jitter)
                })
            }
        } else {
            moves.choose(&mut rng).copied()
        };

        let mv = mv.expect("Rollout failed to find a valid move");
        game.do_move(mv);
        let turn = game.turn;
        let (score, is_terminal) = Self::rollout(game, heuristics, depth + 1);
        heuristics.update_rave(turn, &mv, score);

        (score, is_terminal)
    }
}

impl Default for Edge {
    fn default() -> Self {
        Edge::new(Move::Place(Placement::default()))
    }
}

pub struct MonteCarloTree {
    game: Game,
    pub root: Edge,
    pub heuristics: Heuristics,
}

impl MonteCarloTree {
    #[must_use]
    pub fn new(game: Game) -> Self {
        let root = Edge::default();
        let heuristics = Heuristics::default();
        Self {
            game,
            root,
            heuristics,
        }
    }

    #[must_use]
    pub fn new_with_heuristics(game: Game, heuristics: Heuristics) -> Self {
        Self {
            game,
            heuristics,
            root: Edge::default(),
        }
    }
    /// # Panics
    /// This function panics if the move is not possible in this state of the game.
    pub fn progress(mut mcts: Self, mv: Move, game: &mut Game) -> Self {
        game.do_move(mv);
        if !mcts.heuristics.tree_reuse {
            return Self::new_with_heuristics(game.clone(), mcts.heuristics);
        }

        if matches!(mv, Move::Roll) {
            mcts.game = game.clone();
        } else {
            mcts.game.do_move(mv);
        }

        let move_to_match = if matches!(mv, Move::SetRoll(..)) {
            Move::Roll
        } else {
            mv
        };

        mcts.root = match mcts.root.child.as_mut() {
            None => Edge::default(),
            Some(child) => match child {
                Multiple(_) => {
                    panic!("Could not `progress`. Root edge has child of type Some(Multiple)")
                }
                Single(node) => {
                    // `node` is where we're choosing some action from
                    let children: Box<[Edge]> = std::mem::take(&mut node.children);
                    let child_index = children.iter().position(|child| child.mv == move_to_match);
                    match child_index {
                        None => Edge::default(),
                        Some(index) => {
                            let mut next = Vec::from(children).swap_remove(index);
                            match next.child.as_mut() {
                                None => Edge::default(),
                                Some(child) => match child {
                                    Single(_) => next,
                                    Multiple(nodes) => {
                                        let roll = match mv {
                                        Move::SetRoll(roll) => roll,
                                        Move::Roll => {
                                          let roll = game
                                            .to_place
                                            .clone()
                                            .try_into()
                                            .unwrap_or_else(|_| panic!("Could not progress. Received Move::Roll, and could not infer roll from game due to incorrect `to_place` length."));
                                          Roll::new(roll)
                                        },
                                        _ => panic!("MCTS cannot progress to a Some(Multiple)-child unless the move is a knowable Roll. Received {:?}", &mv),
                                      };

                                        match nodes.remove(&roll) {
                                            None => Edge::default(),
                                            Some(node) => Edge {
                                                mv: Move::SetRoll(roll),
                                                visits: node.visits,
                                                mean_score: node.visits as f64 / node.total_score,
                                                child: Some(Single(node)),
                                            },
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            },
        };

        mcts
    }

    /// Run one iteration of MCTS.
    /// TODO: Implement "move stack" for game that can be used to
    /// undo history and avoid cloning the board every iteration.
    /// Validate that strategy with criterion. If it's not actually
    /// faster, maybe just revert.
    /// Although, having undo-capabilities would be nice for frontend ...
    pub fn search(&mut self) {
        self.root.select(self.game.clone(), &mut self.heuristics);
    }

    pub fn search_iterations(&mut self, iterations: u64) -> &Self {
        for _ in 0..iterations {
            self.search();
        }
        self
    }

    pub fn search_duration(&mut self, milliseconds: u128) {
        let start = std::time::Instant::now();
        while start.elapsed().as_millis() < milliseconds {
            self.search();
            // println!("{:?}", self.root.mv);
        }
    }

    /// Return the best move given the current state of search
    /// # Panics
    /// Panics if no move could be selected from the current game position.
    #[must_use]
    pub fn best_move(&self) -> Move {
        match self.root.child.as_ref() {
            None => *self
                .game
                .generate_moves()
                .choose(&mut rand::thread_rng())
                .expect("Found no moves"),
            Some(child) => match child {
                Multiple(_) => Move::Roll,
                Single(node) => {
                    if let Some(child) = node
                        .children
                        .iter()
                        .filter(|edge| edge.visits != 0)
                        .max_by_key(|edge| edge.visits)
                    {
                        child.mv
                    } else if let Some(mv) =
                        self.game.generate_moves().choose(&mut rand::thread_rng())
                    {
                        *mv
                    } else {
                        Move::End
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn test_single_iteration_search() {
        let game = Game::new();
        let mut tree = MonteCarloTree::new(game);
        tree.search();
        tree.best_move();
    }

    #[test]
    fn test_many_iteration_search() {
        let mut game = Game::new();
        game.roll();
        let mut mcts = MonteCarloTree::new(game);
        mcts.search_iterations(100);
    }

    #[test]
    fn test_play_full_game() {
        let mut game = Game::new();
        game.roll();
        let mut mcts = MonteCarloTree::new(game.clone());
        while !game.ended {
            mcts.search_iterations(100);
            let mv = mcts.best_move();
            mcts = MonteCarloTree::progress(mcts, mv, &mut game);
        }

        println!("{}", game.encode());
        println!("MCTS SCORE {}", game.board.score());
    }

    #[test]
    fn test_play_full_game_duration() {
        let mut game = Game::new();
        game.roll();
        let mut mcts = MonteCarloTree::new(game.clone());
        while !game.ended {
            mcts.search_duration(200);
            let mv = mcts.best_move();
            mcts = MonteCarloTree::progress(mcts, mv, &mut game);
        }
    }
}
