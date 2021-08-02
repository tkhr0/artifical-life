mod utils;

extern crate wasm_bindgen;
extern crate web_sys;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::f32;
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
#[derive(Clone, Copy, Hash, PartialEq)]
pub enum Species {
    Plant,
    // 草食動物
    HERBIVORE,
    // 肉食動物
    CARNIVORE,
}

impl Eq for Species {}

#[wasm_bindgen]
pub struct Life {
    species: Species,
    x: u32,
    y: u32,
    // 直径
    size: u32,
    rng: rand::prelude::ThreadRng,
    direction: Option<Direction>,
    color: &'static str,
}

const DEFAULT_LIFE_SIZE: u32 = 10;

impl Life {
    pub fn new(species: Species, x: u32, y: u32) -> Self {
        Self {
            species: species,
            x: x,
            y: y,
            size: DEFAULT_LIFE_SIZE,
            rng: rand::thread_rng(),
            direction: None,
            color: Self::get_color(species),
        }
    }

    fn get_color(species: Species) -> &'static str {
        match species {
            Species::Plant => "#02ab83",
            Species::HERBIVORE => "#eac435",
            Species::CARNIVORE => "#fb4d3d",
        }
    }

    pub fn next_step(&mut self, field: &Field) {
        if self.change_direction() {
            self.set_direction(rand::random());
        }

        // 進む距離
        const D: u32 = 1;
        let half_of_size: u32 = (self.size as f32 / 2.0f32).ceil() as u32;
        match self.direction {
            Some(Direction::NORTH) => {
                let top = self.y as i32 - half_of_size as i32;
                if (top - D as i32).is_positive() {
                    self.y -= D;
                }
            }
            Some(Direction::EAST) => {
                let right_side = self.x + half_of_size;
                if right_side + D < field.width {
                    self.x += D;
                }
            }
            Some(Direction::SOUTH) => {
                let bottom = self.y + half_of_size;
                if bottom + D < field.height {
                    self.y += D;
                }
            }
            Some(Direction::WEST) => {
                let left_side = self.x as i32 - half_of_size as i32;

                if (left_side - D as i32).is_positive() {
                    self.x -= D;
                }
            }
            _ => {}
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
            (self.size as f64) / 2.0,
            0.0,
            std::f64::consts::PI * 2.0,
            self.color,
            self.color,
        );
    }

    /// 円周上の点座標を返す
    /// degree: 角度（度）
    // TODO: test
    fn get_periphery_point(&self, degree: f32) -> Point {
        let rad = degree * f32::consts::PI / 180_f32;
        let r = self.size as f32 / 2_f32;

        let x = self.x as f32 + r * rad.sin();
        let y = self.y as f32 + r * rad.cos();

        Point {
            x: x as u32,
            y: y as u32,
        }
    }
}

impl Collision for Life {
    fn get_up_left_point(&self) -> Point {
        self.get_periphery_point(135.0)
    }

    fn get_down_right_point(&self) -> Point {
        self.get_periphery_point(315.0)
    }
}

#[wasm_bindgen]
pub struct Field {
    width: u32,
    height: u32,
}

const LENGTH_OF_A_WORLD_SIDE: u32 = 500;
const WORLD_WIDTH: u32 = LENGTH_OF_A_WORLD_SIDE;
const WORLD_HEIGHT: u32 = LENGTH_OF_A_WORLD_SIDE;
const QUAD_TREE_LEVEL_COUNT: usize = 4;

/// 座標軸上の 1点を表す
#[wasm_bindgen]
pub struct Point {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
pub struct CollisionDetector {
    quad_tree: QuadTree,
}

#[wasm_bindgen]
pub struct QuadTree {
    world_width: u32,
    world_height: u32,
    spaces: Vec<Option<Rc<RefCell<QuadTreeSpace>>>>,
}

// #[wasm_bindgen]
// TODO: remove
// pub struct QuadTreeLevel {
//     spaces: Vec<QuadTreeSpace>,
// }

#[wasm_bindgen]
pub struct QuadTreeSpace {
    head_node: Rc<RefCell<Option<QuadTreeNode>>>,
}

#[wasm_bindgen]
pub struct QuadTreeNode {
    belongs_to: Rc<RefCell<Option<QuadTreeSpace>>>,
    life: Rc<RefCell<Life>>,
    prev_node: Rc<RefCell<Option<QuadTreeNode>>>,
    next_node: Rc<RefCell<Option<QuadTreeNode>>>,
}

trait Collision {
    fn get_up_left_point(&self) -> Point;
    fn get_down_right_point(&self) -> Point;
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl CollisionDetector {
    pub fn new(world_width: u32, world_height: u32) -> Self {
        Self {
            quad_tree: QuadTree::new(world_width, world_height),
        }
    }

    pub fn add_member(&mut self, life: Rc<RefCell<Life>>) {
        self.quad_tree.add_member(life);
    }
}

impl QuadTree {
    pub fn new(world_width: u32, world_height: u32) -> Self {
        let mut spaces: Vec<Rc<RefCell<QuadTreeSpace>>> = vec![];
        for i in 0..QUAD_TREE_LEVEL_COUNT {
            for _ in 0..2usize.pow(i as u32).pow(2u32) {
                let space = QuadTreeSpace::new(Rc::new(RefCell::new(None)));
                spaces.push(Rc::new(RefCell::new(space)));
            }
        }

        Self {
            world_width,
            world_height,
            spaces,
        }
    }

    pub fn add_member(&mut self, life: Rc<RefCell<Life>>) {
        let mut space_index: usize = 0;
        {
            space_index = self.get_belongs_to_space_index(life.as_ref().borrow());
        }
        let space: Rc<RefCell<Option<QuadTreeSpace>>> = self.spaces[space_index];

        let cloned_space: Rc<RefCell<Option<QuadTreeSpace>>> = Rc::clone(&space);
        let node: Rc<RefCell<Option<QuadTreeNode>>> = Rc::new(RefCell::new(Some(
            QuadTreeNode::new(cloned_space, Rc::clone(&life), None, None),
        )));

        if space.head_node.is_none() {
            space.head_node = Some(Rc::clone(node));
        } else {
            let second_node = space.head_node.unwrap();
            second_node.prev_node = Some(node.clone());
            node.next_node = Some(second_node.clone());
        }
    }

    fn get_belongs_to_space_index(&self, life: Ref<Life>) -> usize {
        const MAX_LEVEL_INDEX: u32 = QUAD_TREE_LEVEL_COUNT as u32 - 1;

        let up_left_point = life.get_up_left_point();
        let up_left_index = up_left_point.to_quad_tree_index(MAX_LEVEL_INDEX);

        let down_right_point = life.get_down_right_point();
        let down_right_index = down_right_point.to_quad_tree_index(MAX_LEVEL_INDEX);

        (up_left_index ^ down_right_index) as usize

        // let mut index = up_left_index ^ down_right_index;
        //
        // let prev_level = MAX_LEVEL_INDEX - 1;
        // let mut on_level = 0;
        // for level in prev_level..=0 {
        //     let lower_2bit = index & 0x3;
        //     if 0 < lower_2bit {
        //         on_level = level;
        //     }
        //     index >>= 2;
        // }
        //
        // on_level
    }
}

impl Point {
    /// モートン空間でのインデックスに変換する
    /// level で階層を指定する
    pub fn to_quad_tree_index(&self, level: u32) -> u32 {
        let unit_length = 2_u32.pow(level).pow(2);

        let x: u32 = self.x / unit_length;
        let y: u32 = self.y / unit_length;

        x | y
    }
}

// impl QuadTreeLevel {
//     pub fn new(tree_space_count: usize) -> Self {
//         let mut spaces = vec![];
//         for _ in 0..tree_space_count {
//             let space = QuadTreeSpace::new(None);
//             spaces.push(space)
//         }
//
//         Self { spaces }
//     }
// }

impl QuadTreeSpace {
    pub fn new(head_node: Rc<RefCell<Option<QuadTreeNode>>>) -> Self {
        Self { head_node }
    }
}

impl QuadTreeNode {
    pub fn new(
        belongs_to: Rc<RefCell<Option<QuadTreeSpace>>>,
        life: Rc<RefCell<Life>>,
        prev_node: Rc<RefCell<Option<QuadTreeNode>>>,
        next_node: Rc<RefCell<Option<QuadTreeNode>>>,
    ) -> Self {
        Self {
            belongs_to,
            life,
            prev_node,
            next_node,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    field: Field,
    lives: Vec<Rc<RefCell<Life>>>,
    collision_detector: CollisionDetector,
    renderer: Renderer,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(
        width: u32,
        height: u32,
        collision_detector: CollisionDetector,
        renderer: Renderer,
    ) -> Self {
        let field = Field { width, height };
        let lives: Vec<Rc<RefCell<Life>>> = vec![];

        Self {
            field,
            lives,
            collision_detector,
            renderer,
        }
    }

    pub fn birth(&mut self, species: Species, num: u32) {
        let mut rng = rand::thread_rng();
        // half of size
        let hos = (DEFAULT_LIFE_SIZE as f32 / 2.0f32).ceil() as u32;

        let mut lives: Vec<Rc<RefCell<Life>>> = (0..num)
            .map(|_| {
                Rc::new(RefCell::new(Life::new(
                    species,
                    rng.gen_range(hos, self.field.width - hos),
                    rng.gen_range(hos, self.field.height - hos),
                )))
            })
            .collect();
        self.lives.append(&mut lives);
        lives.iter().for_each(|life| {
            self.collision_detector.add_member(Rc::clone(life));
        });
    }

    pub fn next_step(&mut self) {
        let field = &self.field;
        self.lives.iter_mut().for_each(move |life| {
            Rc::get_mut(life).unwrap().next_step(field);
        })
    }

    pub fn render(&self) {
        self.renderer.fill_rect(
            0.0,
            0.0,
            self.field.width as f64,
            self.field.height as f64,
            "#fff",
        );
        self.renderer.stroke_rect(
            0.0,
            0.0,
            self.field.width as f64,
            self.field.height as f64,
            "#000",
        );

        self.lives.iter().for_each(|life| {
            life.as_ref().render(&self.renderer);
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
    let collision_detector = CollisionDetector::new(WORLD_WIDTH, WORLD_HEIGHT);

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

    let mut universe = Universe::new(WORLD_WIDTH, WORLD_HEIGHT, collision_detector, renderer);

    let mut seeds: HashMap<Species, u32> = HashMap::new();
    seeds.insert(Species::Plant, 200);
    seeds.insert(Species::HERBIVORE, 100);
    seeds.insert(Species::CARNIVORE, 10);
    seeds.iter().for_each(|(species, num)| {
        universe.birth(*species, *num);
    });

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
