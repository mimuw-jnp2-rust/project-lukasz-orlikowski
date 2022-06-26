use std::num::ParseIntError;

use gloo_net::Error;
use gloo_storage::errors::StorageError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function get_value(input) { return document.getElementById(input).value; }"
)]
extern "C" {
    pub fn get_value(input: &str) -> String;
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

pub fn map_result(result: Result<i32, ParseIntError>) -> Option<i32> {
    match result {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

#[wasm_bindgen(inline_js = "export function get_parameter(input) { 
        var url_string = window.location.href;
        var url = new URL(url_string);
        var par = url.searchParams.get(input);
        return par;
     }")]

extern "C" {
    pub fn get_parameter(input: &str) -> String;
}

#[wasm_bindgen(inline_js = "export function open_modal(input) { 
        var modal = document.getElementById(input);
        modal.style.display = \"block\";
        return true;
     }")]

extern "C" {
    pub fn open_modal(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function hide_modal(input) { 
        var modal = document.getElementById(input);
        modal.style.display = \"none\";
        return true;
     }")]

extern "C" {
    pub fn hide_modal(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function set_value(input, value) { 
        var el = document.getElementById(input);
        el.value = value;
        return true;
     }")]

extern "C" {
    pub fn set_value(input: &str, value: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function reload() { 
        location.reload()
        return true;
     }")]

extern "C" {
    pub fn reload() -> bool;
}

#[wasm_bindgen(inline_js = "export function set_checked(input) { 
    var el = document.getElementById(input);
    el.checked = true;
    return true;
 }")]

extern "C" {
    pub fn set_checked(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function is_checked(input) { 
    var el = document.getElementById(input);
    if (el.checked) {
        return 1;
    }
    else {
        return 0;
    }
 }")]

extern "C" {
    pub fn is_checked(input: &str) -> i32;
}

#[wasm_bindgen(inline_js = "export function err(input) { 
    alert(input);
    return true;
 }")]

extern "C" {
    pub fn err(input: &str) -> bool;
}

#[wasm_bindgen(inline_js = "export function get_backend() { 
    return window.location.origin + \"/api/\";
 }")]

extern "C" {
    pub fn get_backend() -> String;
}
