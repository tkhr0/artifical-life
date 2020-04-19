mod utils;

extern crate wasm_bindgen;
extern crate web_sys;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
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

const DEBUG: bool = false;

fn debug(message: &str) {
    if DEBUG {
        log(message);
    }
}

#[wasm_bindgen]
pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d,
}

impl Renderer {
    /// Renderer のコンストラクタ
    ///
    /// 2次元の canvas に描画する
    pub fn new(context: web_sys::CanvasRenderingContext2d) -> Self {
        Self { context }
    }

    /// 塗りつぶした四角形を描画する
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64, fill_color: &str) {
        self.context.set_fill_style(&JsValue::from_str(fill_color));
        self.context.fill_rect(x, y, w, h);
    }

    /// 四角形を描画する
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64, stroke_color: &str) {
        self.context
            .set_stroke_style(&JsValue::from_str(stroke_color));
        self.context.stroke_rect(x, y, w, h);
    }

    /// 円を描画する
    pub fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        stroke_color: &str,
        fill_color: &str,
    ) {
        self.context
            .set_stroke_style(&JsValue::from_str(stroke_color));
        self.context.set_fill_style(&JsValue::from_str(fill_color));

        self.context.begin_path();
        self.context
            .arc(x, y, radius, start_angle, end_angle)
            .unwrap();
        self.context.fill();
        self.context.stroke();
    }
}

/// 進行方向を表す列挙型
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::NORTH,
            1 => Direction::EAST,
            2 => Direction::SOUTH,
            _ => Direction::WEST,
        }
    }
}

#[wasm_bindgen]
pub struct Life {
    x: u32,
    y: u32,
    rng: rand::prelude::ThreadRng,
    direction: Option<Direction>,
    color: &'static str,
}

impl Life {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x,
            y: y,
            rng: rand::thread_rng(),
            direction: None,
            color: "#192",
        }
    }

    pub fn next_step(&mut self, width: u32, height: u32) {
        if self.change_direction() {
            self.set_direction(rand::random());
        }

        let dx: i32 = match self.direction {
            Some(Direction::EAST) => 1,
            Some(Direction::WEST) => -1,
            _ => 0,
        };
        let dy: i32 = match self.direction {
            Some(Direction::NORTH) => -1,
            Some(Direction::SOUTH) => 1,
            _ => 0,
        };

        if (dx < 0 && 0 < self.x) || (0 < dx && self.x < width) {
            self.x = ((self.x as i32) + dx) as u32;
        }
        if (dy < 0 && 0 < self.y) || (0 < dy && self.y < height) {
            self.y = ((self.y as i32) + dy) as u32;
        }
        debug(&self.x.to_string());
    }

    /// 方向を変えるかどうか
    pub fn change_direction(&mut self) -> bool {
        self.rng.gen_bool(1.0 / 10.0)
    }

    /// 進行方向を決める
    pub fn set_direction(&mut self, direction: Option<Direction>) {
        self.direction = direction
    }

    pub fn render(&self, renderer: &Renderer) {
        renderer.arc(
            self.x as f64,
            self.y as f64,
            5.0,
            0.0,
            std::f64::consts::PI * 2.0,
            self.color,
            self.color,
        );
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    lives: Vec<Life>,
    renderer: Renderer,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32, renderer: Renderer) -> Self {
        let lives: Vec<Life> = vec![];

        Self {
            width,
            height,
            lives,
            renderer,
        }
    }

    pub fn birth(&mut self, num: u32) {
        let mut rng = rand::thread_rng();

        self.lives = (0..num)
            .map(|_| Life::new(rng.gen_range(0, self.width), rng.gen_range(0, self.height)))
            .collect();
    }

    pub fn next_step(&mut self) {
        let width = self.width;
        let height = self.height;

        self.lives.iter_mut().for_each(|life| {
            life.next_step(width, height);
        })
    }

    pub fn render(&self) {
        self.renderer
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64, "#fff");
        self.renderer
            .stroke_rect(0.0, 0.0, self.width as f64, self.height as f64, "#000");

        self.lives.iter().for_each(|life| {
            life.render(&self.renderer);
        })
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
    const WORLD_WIDTH: u32 = 500;
    const WORLD_HEIGHT: u32 = 500;

    let element = document()
        .get_element_by_id("canvas-universe")
        .expect("not found `canvas`");
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(((WORLD_WIDTH as f32) * 1.2) as u32);
    canvas.set_height(((WORLD_HEIGHT as f32) * 1.2) as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let renderer = Renderer::new(context);

    let mut universe = Universe::new(WORLD_WIDTH, WORLD_HEIGHT, renderer);
    universe.birth(300);
    render_loop(universe);
}

fn render_loop(universe: Universe) {
    {
        let univ = Rc::new(RefCell::new(universe)).clone();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            {
                univ.borrow_mut().next_step();
            }

            (univ.borrow() as &RefCell<Universe>).borrow().render();

            request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.as_ref().borrow().as_ref().unwrap());
    }
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
