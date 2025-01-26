use crate::console_log;
use crate::game::Game;
use crate::mcts::MonteCarloTree;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameController {
    game: Game,
    mcts: Option<MonteCarloTree>,
}

impl Default for GameController {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GameController {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        set_panic_hook();

        Self {
            game: Game::default(),
            mcts: None,
        }
    }

    #[must_use]
    pub fn encode(&self) -> String {
        self.game.encode()
    }

    /// # Panics
    /// Panics if serde can't deserialize
    ///
    /// # Errors
    /// Returns an error if the string can't be decoded
    pub fn decode(&mut self, string: JsValue) -> Result<bool, JsValue> {
        let game_str: Result<String, String> = serde_wasm_bindgen::from_value(string)?;
        if let Ok(game_str) = game_str {
            match Game::decode(&game_str) {
                Err(message) => Err(serde_wasm_bindgen::to_value(&message).unwrap()),
                Ok(game) => {
                    self.game = game;
                    self.mcts = None;
                    Ok(true)
                }
            }
        } else {
            Err(serde_wasm_bindgen::to_value("Error decoding game").unwrap())
        }
    }

    #[must_use]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn get(&self) -> JsValue {
        // FIXME: this doesn't work in wasm-land.
        // Probably can't serialize the Board because of the new frontier-thing. Maybe just drop it.
        serde_wasm_bindgen::to_value(&self.game).unwrap()
    }

    /// # Panics
    /// Panics if serde can't serialize
    pub fn roll(&mut self) -> JsValue {
        let pieces = self.game.roll();
        serde_wasm_bindgen::to_value(&pieces).unwrap()
    }

    #[must_use]
    #[wasm_bindgen(js_name = findPossible)]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn find_possible(&self, piece: u8) -> JsValue {
        let candidates = self.game.board.find_possible(piece);
        serde_wasm_bindgen::to_value(&candidates).unwrap()
    }

    /// # Panics
    /// Panics if serde can't serialize
    ///
    /// # Errors
    /// Returns an error if the piece can't be found
    pub fn place(&mut self, placement: JsValue) -> Result<u8, JsValue> {
        let placement = serde_wasm_bindgen::from_value(placement)?;
        match self.game.place(placement) {
            Ok(piece) => {
                self.mcts = None;
                Ok(piece)
            }
            Err(message) => Err(serde_wasm_bindgen::to_value(&message).unwrap()),
        }
    }

    #[must_use]
    pub fn score(&self) -> i32 {
        self.game.board.score()
    }

    /// # Panics
    /// Panics if serde can't serialize
    pub fn search(&mut self) -> JsValue {
        if self.mcts.is_none() {
            self.mcts = Some(MonteCarloTree::new(self.game.clone()));
        }

        let mcts = self.mcts.as_mut().unwrap();
        mcts.search();
        serde_wasm_bindgen::to_value(&mcts.best_move()).unwrap()
    }

    #[wasm_bindgen(js_name = searchFor)]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn search_for(&mut self, iterations: u32) -> JsValue {
        if self.mcts.is_none() {
            self.mcts = Some(MonteCarloTree::new(self.game.clone()));
        }

        let mcts = self.mcts.as_mut().unwrap();
        mcts.search_iterations(u64::from(iterations));
        let mv = mcts.best_move();
        console_log!("{:?}", mv);
        serde_wasm_bindgen::to_value(&mv).unwrap()
    }

    pub fn autoplay(&mut self, iterations: u32) {
        let mut mcts = MonteCarloTree::new(self.game.clone());
        mcts.search_iterations(u64::from(iterations));
        let mv = mcts.best_move();
        self.game.do_move(mv);
    }
}
