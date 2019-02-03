extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use core::f64::consts::PI;
use glutin_window::GlutinWindow as Window;
use graphics::character::CharacterCache;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::char;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ball_x: f64,
    ball_y: f64,
    left_paddle: f64,
    right_paddle: f64,
    ball_direction: f64,
    score: [u32; 2]
}

impl App {
    fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            ball_x: 0.0,
            ball_y: 0.0,
            left_paddle: 0.0, 
            right_paddle: 0.0,
            ball_direction: 30.0,
            score: [0, 0]
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY:   [f32; 4] = [0.6, 0.6, 0.6, 1.0];

        const BALL_SIZE: f64 = 20.0;

        let (center_x, center_y) = (args.width / 2.0,
                      args.height / 2.0);
        let ball= rectangle::square(self.ball_x, self.ball_y, BALL_SIZE);

        let _left_score=char::from_digit(self.score[0], 10);
        let _right_score=char::from_digit(self.score[1], 10);
        // Set me up the font
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let ref mut glyphs = GlyphCache::new("assets/Square.ttf", (), texture_settings)
           .expect("Could not load font");


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draw a box rotating around the middle of the screen.
            for i in 1 .. 20 {
                let strip_y=(args.height / BALL_SIZE) * i as f64;
                let strip = rectangle::square(center_x, strip_y, BALL_SIZE);
                rectangle(GRAY, strip, c.transform, gl);
            }
            rectangle(WHITE, ball, c.transform, gl);
            // Draw the score 
            let text_image = Image::new_color(WHITE);

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // move ball
        self.ball_x = self.ball_x + 10.0 * (self.ball_direction * PI / 180.0).cos();
        self.ball_y = self.ball_y + 10.0 * (self.ball_direction* PI / 180.0).sin();

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "pong",
            [1920, 1080]
        )
        .fullscreen(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
