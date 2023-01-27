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

    /// Panics if serde can't deserialize
    pub fn decode(&mut self, string: &JsValue) -> Result<bool, JsValue> {
        match Game::decode(string.into_serde::<String>().unwrap().as_str()) {
            Err(message) => Err(JsValue::from_serde(&message).unwrap()),
            Ok(game) => {
                self.game = game;
                self.mcts = None;
                Ok(true)
            }
        }
    }

    #[must_use]
    /// Panics if serde can't serialize
    pub fn get(&self) -> JsValue {
        // FIXME: this doesn't work in wasm-land.
        // Probably can't serialize the Board because of the new frontier-thing. Maybe just drop it.
        JsValue::from_serde(&self.game).unwrap()
    }

    /// Panics if serde can't serialize
    pub fn roll(&mut self) -> JsValue {
        let pieces = self.game.roll();
        JsValue::from_serde(&pieces).unwrap()
    }

    #[must_use]
    #[wasm_bindgen(js_name = findPossible)]
    /// Panics if serde can't serialize
    pub fn find_possible(&self, piece: u8) -> JsValue {
        let candidates = self.game.board.find_possible(piece);
        JsValue::from_serde(&candidates).unwrap()
    }

    /// Panics if serde can't serialize
    pub fn place(&mut self, placement: &JsValue) -> Result<u8, JsValue> {
        match self.game.place(placement.into_serde().unwrap()) {
            Ok(piece) => {
                self.mcts = None;
                Ok(piece)
            }
            Err(message) => Err(JsValue::from_serde(&message).unwrap()),
        }
    }

    #[must_use]
    pub fn score(&self) -> i32 {
        self.game.board.score()
    }

    /// Panics if serde can't serialize
    pub fn search(&mut self) -> JsValue {
        if self.mcts.is_none() {
            self.mcts = Some(MonteCarloTree::new(self.game.clone()));
        }

        let mcts = self.mcts.as_mut().unwrap();
        mcts.search();
        JsValue::from_serde(&mcts.best_move()).unwrap()
    }

    #[wasm_bindgen(js_name = searchFor)]
    /// Panics if serde can't serialize
    pub fn search_for(&mut self, iterations: u32) -> JsValue {
        if self.mcts.is_none() {
            self.mcts = Some(MonteCarloTree::new(self.game.clone()));
        }

        let mcts = self.mcts.as_mut().unwrap();
        mcts.search_iterations(u64::from(iterations));
        let mv = mcts.best_move();
        console_log!("{:?}", mv);
        JsValue::from_serde(&mv).unwrap()
    }

    pub fn autoplay(&mut self, iterations: u32) {
        let mut mcts = MonteCarloTree::new(self.game.clone());
        mcts.search_iterations(u64::from(iterations));
        let mv = mcts.best_move();
        self.game.do_move(mv);
    }
}
