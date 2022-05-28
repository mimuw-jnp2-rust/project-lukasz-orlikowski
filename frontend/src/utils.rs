use gloo_net::Error;
use gloo_storage::errors::StorageError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function getValue(input) { return document.getElementById(input).value; }"
)]
extern "C" {
    pub fn getValue(input: &str) -> String;
}

pub enum Msg<T> {
    Submit,
    Res(Result<T, Error>)
}

pub fn map_token(token: Result<String, StorageError>) -> Option<String>{
    match token {
        Ok(key) => Some(key),
        Err(_) => None
    }
}