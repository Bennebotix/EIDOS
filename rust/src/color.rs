use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ColorPalette {
    colors: Vec<u32>,
}

#[wasm_bindgen]
impl ColorPalette {
    pub fn get_colors(&self) -> Vec<u32> {
        self.colors.clone()
    }
}
