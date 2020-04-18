mod utils;

extern crate wasm_bindgen;
extern crate web_sys;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
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
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
    pub fn new() -> Self {
        let width = 500;
        let height = 500;
        let lives = (0..20).map(|i| Life { x: i, y: i }).collect();

        Self {
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

    pub fn next_step(&mut self) {
        let width = self.width;
        let height = self.height;

        self.lives.iter_mut().for_each(|life| {
            let mut rng = thread_rng();
            let direction: u32 = rng.gen_range(0, 4);
            log(&direction.to_string());

            let dx: i32 = match direction {
                1 => 1,
                3 => -1,
                _ => 0,
            };
            let dy: i32 = match direction {
                0 => -1,
                2 => 1,
                _ => 0,
            };

            if (dx < 0 && 0 < life.x) || (0 < dx && life.x < width) {
                life.x = ((life.x as i32) + dx) as u32;
            }
            if (dy < 0 && 0 < life.y) || (0 < dy && life.y < height) {
                life.y = ((life.y as i32) + dy) as u32;
            }
            log(&life.x.to_string());
        })
    }

    pub fn render(&self, context: web_sys::CanvasRenderingContext2d) {
        let tmp_stroke_style = context.stroke_style();
        let tmp_fill_style = context.fill_style();
        let color = &JsValue::from_str("#192");

        context.set_stroke_style(color);
        context.set_fill_style(color);

        self.lives.iter().for_each(|life| {
            context.begin_path();
            context
                .arc(
                    life.x as f64,
                    life.y as f64,
                    5.0,
                    0.0,
                    std::f64::consts::PI * 2.0,
                )
                .unwrap();
            context.fill();
            context.stroke();
        });

        context.set_stroke_style(&tmp_stroke_style);
        context.set_fill_style(&tmp_fill_style);
    }
}

// debug function
// usage: log(&typeof(hoge))
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

#[wasm_bindgen]
pub fn start() {
    let mut universe = Universe::new();

    universe.lives = (1..20)
        .map(|i| Life {
            x: i * 10,
            y: i * 10,
        })
        .collect();

    let element = document()
        .get_element_by_id("canvas-universe")
        .expect("not found `canvas`");
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(((universe.width as f32) * 1.2) as u32);
    canvas.set_height(((universe.height as f32) * 1.2) as u32);

    render_loop(universe);
}

fn render_loop(universe: Universe) {
    {
        let width = universe.width as f64;
        let height = universe.height as f64;
        let univ = Rc::new(RefCell::new(universe)).clone();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            draw_field(width, height);
            {
                univ.borrow_mut().next_step();
            }

            (univ.borrow() as &RefCell<Universe>)
                .borrow()
                .render(get_canvas_context());

            request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.as_ref().borrow().as_ref().unwrap());
    }
}

fn draw_field(width: f64, height: f64) {
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

    ctx.set_fill_style(&JsValue::from_str("#fff"));
    ctx.fill_rect(0.0, 0.0, width, height);
    ctx.set_stroke_style(&JsValue::from_str("#000"));
    ctx.stroke_rect(0.0, 0.0, width, height);
    ctx.stroke();
}

fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let element = document()
        .get_element_by_id("canvas-universe")
        .expect("not found `canvas`");
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
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
