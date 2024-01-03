use debug_log::debug_dbg;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct MyStruct {
    value: u8,
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn enableDebug(s: &str) {
    debug_log::set_debug(s);
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn log(&self) {
        debug_log::group!("group");
        debug_log::debug_log!("haha");
        debug_dbg!(self);
    }
}
