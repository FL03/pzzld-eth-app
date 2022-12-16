#[cfg(test)]
use wasm::greet;

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn lib_compiles() {
    let f = |i: usize| i + 1;

    assert_eq!(f(10), 11)
}

#[wasm_bindgen_test]
fn greet_test() {
    let a = greet("");
    assert!(true);
}
