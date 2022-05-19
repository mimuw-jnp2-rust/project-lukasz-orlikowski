use gloo_net::Error;
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