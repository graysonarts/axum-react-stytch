use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod auth;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserClassification {
    Admin,
    User,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub auth_id: String,
    pub stytch_id: String,
    pub classification: UserClassification,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(auth_id: &str, classification: UserClassification, stytch_id: &str) -> Self {
        Self {
            auth_id: auth_id.to_owned(),
            classification,
            stytch_id: stytch_id.to_owned(),
        }
    }
}
