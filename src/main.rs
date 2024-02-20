use rand::{self, seq::SliceRandom};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 600;
const BACKGROUND_COLOUR: Color = Color::RGB(50, 50, 50);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Sorting algorithm", WINDOW_WIDTH + 10, WINDOW_HEIGHT + 10)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut arr = Array::new();
    let mut done = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if !done {
            arr.algorithm(&mut canvas);
            done = true;
        }

        arr.draw_box(&mut canvas, None);
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60));
    }
}

struct Array {
    vector: Vec<u32>,
}

impl Array {
    fn new() -> Self {
        let mut vector: Vec<u32> = (1..=600).collect();
        vector.shuffle(&mut rand::thread_rng());

        Self { vector }
    }

    fn algorithm(&mut self, canvas: &mut Canvas<Window>) {
        let length = self.vector.len();

        for i in 0..length {
            for j in i..length {
                if self.vector[j] < self.vector[i] {
                    self.swap(i, j, canvas);
                }
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize, canvas: &mut Canvas<Window>) {
        self.draw_box(canvas, Some([i, j]));
        (self.vector[j], self.vector[i]) = (self.vector[i], self.vector[j]);
    }

    fn draw_box(&self, canvas: &mut Canvas<Window>, swapped: Option<[usize; 2]>) {
        let default = Color::RGB(200, 200, 200);
        // let selected = Color::RGB(65, 225, 65);
        let swap = Color::RGB(255, 65, 65);

        let length = self.vector.len() as u32;

        let box_width = WINDOW_WIDTH / length;
        let box_height_multiple = WINDOW_HEIGHT / length;

        let mut current_x = 5;

        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();

        for (idx, i) in self.vector.iter().enumerate() {
            let height = box_height_multiple * i;

            canvas.set_draw_color(default);

            if let Some(i) = swapped {
                if i.contains(&idx) {
                    canvas.set_draw_color(swap);
                }
            }

            canvas
                .fill_rect(Rect::new(
                    current_x as i32,
                    (WINDOW_HEIGHT - height) as i32 + 5,
                    box_width,
                    height,
                ))
                .unwrap();

            current_x += box_width;
        }

        canvas.present();
    }
}
