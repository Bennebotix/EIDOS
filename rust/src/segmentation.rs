use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Region {
    pub id: usize,
    pub color: u32,
    pixels: Vec<u32>,
}

impl Region {
    pub fn pixel_count(&self) -> usize {
        self.pixels.len()
    }
    pub fn get_pixels(&self) -> &[u32] {
        &self.pixels
    }
}
