use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub enum GameError {
    JsError(JsValue),
    RepositoryError(String),
}

impl From<JsValue> for GameError {
    fn from(error: JsValue) -> Self {
        GameError::JsError(error)
    }
}
