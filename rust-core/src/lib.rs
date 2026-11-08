pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_greeting(name: &str) -> String {
    format!("こんにちは、{}さん！この文字列はRust（WASM）の魂から出力されています🦀", name)
}