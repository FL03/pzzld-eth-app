/*
    Appellation: utils <wasm>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}

#[wasm_bindgen]
pub fn add_one(data: usize) -> usize {
    data + 1
}

pub struct ERC721 {
    pub name: String,
    pub symbol: String,
    pub total_supply: usize
}
