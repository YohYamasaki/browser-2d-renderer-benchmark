use vello::{
    Scene,
    kurbo::{Affine, Rect},
    peniko::Color,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

const BOX_SIZE: f64 = 4.0;

pub struct BouncingRect {
    box_x: f64,
    box_y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl BouncingRect {
    pub fn new(velocity_x: f64, velocity_y: f64, canvas_width: f64, canvas_height: f64) -> Self {
        Self {
            box_x: canvas_width * random(),
            box_y: canvas_height * random(),
            velocity_x,
            velocity_y,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        if self.box_x <= 0.0 || self.box_x + BOX_SIZE > width {
            self.velocity_x *= -1.0;
        }
        if self.box_y <= 0.0 || self.box_y + BOX_SIZE > height {
            self.velocity_y *= -1.0;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    pub fn draw(&self, scene: &mut Scene) {
        let rect = Rect::new(
            self.box_x,
            self.box_y,
            self.box_x + BOX_SIZE,
            self.box_y + BOX_SIZE,
        );
        let rect_fill_color = Color::new([0.3, 0.8, 0.3, 0.7]);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            rect_fill_color,
            None,
            &rect,
        );
    }

    pub fn generate_rect(
        rects: &mut Vec<BouncingRect>,
        num: i32,
        canvas_width: f64,
        canvas_height: f64,
    ) {
        for _ in 0..num {
            let rect = BouncingRect::new(
                get_random_velocity(),
                get_random_velocity(),
                canvas_width,
                canvas_height,
            );
            rects.push(rect);
        }
    }
}

fn get_random_velocity() -> f64 {
    let mut res = random() * 5.0;
    if random() < 0.5 {
        res *= -1.0
    }
    res
}
