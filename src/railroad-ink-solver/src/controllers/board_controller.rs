use crate::board::Board;
use crate::utils::set_panic_hook;
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

        BoardController {
            board: Board::new(),
        }
    }

    #[must_use]
    pub fn encode(&self) -> String {
        self.board.encode()
    }

    #[must_use]
    /// Panics if serde can't serialize
    pub fn decode(string: &JsValue) -> Self {
        Self {
            board: Board::decode(string.into_serde::<String>().unwrap().as_str()),
        }
    }

    #[must_use]
    /// Panics if serde can't serialize
    pub fn get(&self) -> JsValue {
        JsValue::from_serde(&self.board).unwrap()
    }

    #[wasm_bindgen(js_name = findPossible)]
    /// Panics if serde can't serialize
    pub fn find_possible(&self, piece: u8) -> Result<JsValue, JsValue> {
        let candidates = self.board.find_possible(piece);
        Ok(JsValue::from_serde(&candidates).unwrap())
    }

    #[wasm_bindgen(js_name = place)]
    /// Panics if serde can't serialize
    pub fn place(&mut self, placement: &JsValue) {
        self.board.place(placement.into_serde().unwrap());
    }

    #[must_use]
    pub fn score(&self) -> i32 {
        self.board.score()
    }
}
