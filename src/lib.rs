mod utils;

extern crate wasm_bindgen;
extern crate web_sys;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    pub fn lives(&self) -> *const Life {
        self.lives.as_ptr()
    }

    pub fn lives_size(&self) -> usize {
        self.lives.len()
    }
}

#[wasm_bindgen]
pub fn start() {
    render_loop();
}

fn render_loop() {
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            draw_field();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn draw_field() {
    let element = document()
        .get_element_by_id("canvas-universe")
        .expect("not found `canvas`");
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.move_to(0.0, 0.0);
    ctx.line_to(20.0, 0.0);
    ctx.line_to(20.0, 20.0);
    ctx.line_to(0.0, 20.0);
    ctx.close_path();

    ctx.stroke();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no global `document exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
