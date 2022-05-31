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
    Res(Result<T, Error>),
}

pub fn map_token(token: Result<String, StorageError>) -> Option<String> {
    match token {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}

#[wasm_bindgen(inline_js = "export function getParameter(input) { 
        var url_string = window.location.href;
        var url = new URL(url_string);
        var par = url.searchParams.get(input);
        return par;
     }")]

extern "C" {
    pub fn getParameter(input: &str) -> String;
}

#[wasm_bindgen(inline_js = "export function openModal(input) { 
        var modal = document.getElementById(input);
        modal.style.display = \"block\";
        return true;
     }")]

extern "C" {
    pub fn openModal(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function hideModal(input) { 
        var modal = document.getElementById(input);
        modal.style.display = \"none\";
        return true;
     }")]

extern "C" {
    pub fn hideModal(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function setValue(input, value) { 
        var el = document.getElementById(input);
        el.value = value;
        return true;
     }")]

extern "C" {
    pub fn setValue(input: &str, value: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function reload() { 
        location.reload()
        return true;
     }")]

extern "C" {
    pub fn reload() -> bool;
}
