use tiny_skia::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

pub struct BouncingRect {
    box_size: f32,
    box_x: f32,
    box_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl BouncingRect {
    pub fn new(
        box_size: f32,
        velocity_x: f32,
        velocity_y: f32,
        canvas_width: u32,
        canvas_height: u32,
    ) -> Self {
        Self {
            box_size,
            box_x: (canvas_width as f32 - box_size) * random(),
            box_y: (canvas_height as f32 - box_size) * random(),
            velocity_x,
            velocity_y,
        }
    }

    pub fn update(&mut self, width: i16, height: i16) {
        if self.box_x <= 0.0 || self.box_x + self.box_size as f32 > width as f32 {
            self.velocity_x *= -1.0;
        }
        if self.box_y <= 0.0 || self.box_y + self.box_size as f32 > height as f32 {
            self.velocity_y *= -1.0;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    pub fn draw(&self, pixmap: &mut Pixmap) {
        let mut paint2 = Paint::default();
        paint2.set_color_rgba8(100, 200, 100, 200);
        paint2.anti_alias = true;

        let path2 = PathBuilder::from_rect(
            Rect::from_ltrb(
                self.box_x as f32,
                self.box_y as f32,
                self.box_x + self.box_size as f32,
                self.box_y + self.box_size as f32,
            )
            .unwrap(),
        );

        pixmap.fill_path(
            &path2,
            &paint2,
            FillRule::Winding,
            Transform::identity(),
            None,
        )
    }

    pub fn generate_rect(
        rects: &mut Vec<BouncingRect>,
        num: u32,
        box_size: f32,
        canvas_width: u32,
        canvas_height: u32,
    ) {
        for _ in 0..num {
            let rect = BouncingRect::new(
                box_size,
                get_random_velocity(),
                get_random_velocity(),
                canvas_width,
                canvas_height,
            );
            rects.push(rect);
        }
    }
}

fn get_random_velocity() -> f32 {
    let mut res = (random() * 5.0) as f32;
    if random() < 0.5 {
        res *= -1.0
    }
    res
}
