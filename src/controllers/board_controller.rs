use crate::board::placement::Placement;
use crate::board::Board;
use crate::utils::set_panic_hook;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BoardController {
    #[wasm_bindgen(skip)]
    pub board: Board,
}

impl Default for BoardController {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl BoardController {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        set_panic_hook();

        Self {
            board: Board::new(),
        }
    }

    #[must_use]
    pub fn encode(&self) -> String {
        self.board.encode()
    }

    #[must_use]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn decode(string: JsValue) -> Self {
        let board_str: String =
            serde_wasm_bindgen::from_value(string).expect("Error decoding board");
        Self {
            board: Board::decode(board_str.as_str()),
        }
    }

    #[must_use]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn get(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.board).unwrap()
    }

    #[wasm_bindgen(js_name = findPossible)]
    /// # Panics
    /// Panics if serde can't serialize
    ///
    /// # Errors
    /// Returns an error if the piece can't be found
    pub fn find_possible(&self, piece: u8) -> Result<JsValue, JsValue> {
        let candidates = self.board.find_possible(piece);
        Ok(serde_wasm_bindgen::to_value(&candidates).unwrap())
    }

    #[wasm_bindgen(js_name = place)]
    /// # Panics
    /// Panics if serde can't serialize
    pub fn place(&mut self, placement: JsValue) {
        let placement: String =
            serde_wasm_bindgen::from_value(placement).expect("Error decoding board");
        let placement = Placement::from_str(placement.as_str()).expect("Error decoding board");
        self.board.place(placement);
    }

    #[must_use]
    pub fn score(&self) -> i32 {
        self.board.score()
    }
}
