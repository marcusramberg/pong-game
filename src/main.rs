extern crate find_folder;
extern crate piston_window;

use core::f64::consts::PI;
use piston_window::*;

pub struct App {
    window: PistonWindow, // OpenGL drawing backend.
    ball_x: f64,
    ball_y: f64,
    current_width: f64,
    current_height: f64,
    left_paddle: f64,
    right_paddle: f64,
    ball_direction: f64,
    score: [u32; 2],
}

impl App {
    fn new(window: PistonWindow) -> App {
        App {
            window: window,
            ball_x: 0.0,
            ball_y: 0.0,
            current_width: 0.0,
            current_height: 0.0,
            left_paddle: 0.0,
            right_paddle: 0.0,
            ball_direction: 30.0,
            score: [0, 0],
        }
    }

    fn render(&mut self, args: &RenderArgs, glyphs: &mut Glyphs, event: &piston_window::Event) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
        const FONTSIZE: u32 = 48;

        const BALL_SIZE: f64 = 20.0;

        let (center_x, center_y) = (args.width / 2.0, args.height / 2.0);
        let ball = rectangle::square(self.ball_x, self.ball_y, BALL_SIZE);
        let paddle = rectangle::rectangle_by_corners(
            args.width / 10.0,
            self.left_paddle,
            (args.width / 10.0) + 5.0,
            self.left_paddle + 20.0,
        );

        let mut _left_score = self.score[0].to_string();
        let mut _right_score = self.score[1].to_string();
        self.current_height = args.height;
        self.current_width = args.width;

        self.window.draw_2d(event, |c, g| {
            // Clear the screen.
            clear(BLACK, g);

            for i in 1..20 {
                let strip_y = (args.height / BALL_SIZE) * i as f64;
                let strip = rectangle::square(center_x, strip_y, BALL_SIZE);
                rectangle(GRAY, strip, c.transform, g);
            }
            rectangle(WHITE, ball, c.transform, g);
            // Draw the score
            let left_score_pos = c.transform.trans((args.width / 10.0) * 4.0, 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_left_score, glyphs, &c.draw_state, left_score_pos, g)
                .unwrap();
            let right_score_pos = c.transform.trans((args.width / 10.0) * 6.0, 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_right_score, glyphs, &c.draw_state, right_score_pos, g)
                .unwrap();
            //let text_image = Image::new_color(WHITE);
            // text_image.build
        });
    }

    fn run(&mut self) {
        // Load font
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("Square.ttf");
        let factory = self.window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r, &mut glyphs, &e);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // move ball
        self.ball_x = self.ball_x + 10.0 * (self.ball_direction * PI / 180.0).cos();
        self.ball_y = self.ball_y + 10.0 * (self.ball_direction * PI / 180.0).sin();
        if self.current_width > 0.0 && self.ball_x > self.current_width {
            self.score[0] = self.score[0] + 1;
            self.reverse_direction();
        }
        if self.ball_x < 0.0 {
            self.score[1] = self.score[1] + 1;
            self.reverse_direction();
        }
    }

    fn bounce(&mut self) {}

    fn reverse_direction(&mut self) {
        self.ball_direction = self.ball_direction + 180.0;
        if self.ball_direction > 360.0 {
            self.ball_direction = self.ball_direction % 360.0;
        }
    }
}

fn main() {
    // Create an Piston window.
    let mut window: PistonWindow = WindowSettings::new("pong", [1920, 1080])
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    // Create a new game and run it.
    let mut app = App::new(window);

    app.run();
}
