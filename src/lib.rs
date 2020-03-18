mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Life {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    lives: Vec<Life>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 500;
        let height = 500;
        let lives = (0..20).map(|i| Life { x: i, y: i }).collect();

        Universe {
            width: width,
            height: height,
            lives: lives,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
