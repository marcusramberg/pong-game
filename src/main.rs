extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

// Constants
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GRAY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
const FONTSIZE: u32 = 48;
const BALL_SIZE: f64 = 20.0;
const HEIGHT: f64 = 1080.0;
const WIDTH: f64 = 1920.0;

pub struct Pong {
    window: PistonWindow, // OpenGL drawing backend.
    ball_x: f64,
    ball_y: f64,
    left_paddle_top: f64,
    right_paddle_top: f64,
    x_velocity: f64,
    y_velocity: f64,
    left_paddle: [f64; 4],
    right_paddle: [f64; 4],
    score: [u32; 2],
}

impl Pong {
    fn new(window: PistonWindow) -> Pong {
        Pong {
            window: window,
            ball_x: 0.0,
            ball_y: 0.0,
            left_paddle_top: 40.0,
            right_paddle_top: 40.0,
            left_paddle: [0.0, 0.0, 0.0, 0.0],
            right_paddle: [0.0, 0.0, 0.0, 0.0],
            x_velocity: 8.0,
            y_velocity: 8.0,
            score: [0, 0],
        }
    }

    fn render(&mut self, _args: &RenderArgs, glyphs: &mut Glyphs, event: &piston_window::Event) {

        let center_x = _args.width / 2.0;
        let ball = rectangle::square(self.ball_x, self.ball_y, BALL_SIZE);
        let (paddle_width, paddle_height) = (_args.width / 50.0, _args.height / 10.0);
        let left_paddle_pos = _args.width / 20.0;
        let right_paddle_pos = left_paddle_pos * 19.0;
        self.left_paddle = rectangle::rectangle_by_corners(
            left_paddle_pos,
            self.left_paddle_top,
            left_paddle_pos + paddle_width,
            self.left_paddle_top + paddle_height,
        );
        let left_paddle = self.left_paddle;
        self.right_paddle = rectangle::rectangle_by_corners(
            right_paddle_pos,
            self.right_paddle_top,
            right_paddle_pos + paddle_width,
            self.left_paddle_top + paddle_height,
        );
        let right_paddle = self.right_paddle;

        let mut _left_score = self.score[0].to_string();
        let mut _right_score = self.score[1].to_string();

        self.window.draw_2d(event, |c, g| {
            // Clear the screen.
            clear(BLACK, g);

            for i in 1..20 {
                let strip_y = (_args.height / BALL_SIZE) * i as f64;
                let strip = rectangle::square(center_x, strip_y, BALL_SIZE);
                rectangle(GRAY, strip, c.transform, g);
            }
            rectangle(WHITE, ball, c.transform, g);
            rectangle(WHITE, left_paddle, c.transform, g);
            rectangle(WHITE, right_paddle, c.transform, g);
            // Draw the score
            let left_score_pos = c.transform.trans((_args.width / 10.0) * 4.0, 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_left_score, glyphs, &c.draw_state, left_score_pos, g)
                .unwrap();
            let right_score_pos = c.transform.trans((_args.width / 10.0) * 6.0, 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_right_score, glyphs, &c.draw_state, right_score_pos, g)
                .unwrap();
        });
    }

    fn reset_ball(&mut self, right: bool) {
        let mut rng = rand::thread_rng();
        if right {
            self.ball_x = WIDTH;
        } else {
            self.ball_x = 0.0;
        }
        self.y_velocity = rng.gen_range(-8.0, 8.0);
         self.ball_y = rng.gen_range(0.0, HEIGHT);
    }

    fn reset_game(&mut self) {
        self.score = [0, 0];
        self.reset_ball(false);
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
        self.reset_game();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r, &mut glyphs, &e);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn hit_paddle(&mut self) {
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // move ball
        self.ball_x = self.ball_x + self.x_velocity;
        self.ball_y = self.ball_y + self.y_velocity;
        // Point for left side
        if self.ball_x > WIDTH {
            self.score[0] = self.score[0] + 1;
            if self.score[0] > 14 {
                self.reset_game();
                return;
            }
            self.x_velocity = 0.0 - self.x_velocity;
            self.reset_ball(true);
            return;
        }
        // Point for right side;
        else if self.ball_x < 0.0 {
            self.score[1] = self.score[1] + 1;
            if self.score[1] > 14 {
                self.reset_game();
                return;
            }
            self.x_velocity = 0.0 - self.x_velocity;
            self.reset_ball(false);
            return;
        }
        if self.ball_y < 0.0 {
            self.y_velocity = 0.0 - self.y_velocity;
        }
        if self.ball_y > HEIGHT {
            self.y_velocity = 0.0 - self.y_velocity;
        }
        println!("{:?}", self.left_paddle);
        if self.x_velocity > 0.0
            && (self.ball_x < self.left_paddle[2])
            && (self.ball_y < self.left_paddle[0])
            && (self.ball_y > self.left_paddle[3])
        {
            println!("OMG left HIT");
        }
    }
}

fn main() {
    // Create an Piston window.
    let mut window: PistonWindow = WindowSettings::new("pong", [HEIGHT, WIDTH])
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    // Create a new game and run it.
    let mut pong= Pong::new(window);
    pong.run();
}
