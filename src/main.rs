extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

// Constants
const HEIGHT: f64 = 768.0;
const WIDTH: f64 = 1366.0;
//const HEIGHT: f64 = 1080.0;
//const WIDTH: f64 = 1920.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GRAY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const BALL_SIZE: f64 = 20.0;
const CELL_COUNT: f64 = 10.0;
const CENTER: f64 = (WIDTH / 2.0);
const FONTSIZE: u32 = 48;
const LEFT_PADDLE_POS: f64 = (WIDTH / 20.0);
const PADDLE_HEIGHT: f64 = (HEIGHT / 10.0);
const PADDLE_VELOCITY: f64 = 8.0;
const PADDLE_WIDTH: f64 = (WIDTH / 50.0);
const RIGHT_PADDLE_POS: f64 = (LEFT_PADDLE_POS * 19.0);
const WINNING_SCORE: u32 = 14;

pub struct Pong {
    ball_x: f64,
    ball_y: f64,
    left_paddle: [f64; 4],
    paddle_top: [f64; 2],
    pressed_keys: [bool; 4],
    right_paddle: [f64; 4],
    score: [u32; 2],
    window: PistonWindow, // OpenGL drawing backend.
    x_velocity: f64,
    y_velocity: f64,
}

impl Pong {
    fn new(window: PistonWindow) -> Pong {
        Pong {
            ball_x: 0.0,
            ball_y: 0.0,
            left_paddle: [0.0, 0.0, 0.0, 0.0],
            paddle_top: [40.0, 40.0],
            pressed_keys: [false, false, false, false],
            right_paddle: [0.0, 0.0, 0.0, 0.0],
            score: [0, 0],
            window: window,
            x_velocity: 8.0,
            y_velocity: 8.0,
        }
    }

    fn hit_paddle(&mut self) -> bool {
        let center_x = self.ball_x + BALL_SIZE / 2.0;
        let center_y = self.ball_y + BALL_SIZE / 2.0;
        if self.x_velocity < 0.0 && hit_rect(self.left_paddle, center_x, center_y) {
            self.y_velocity = self.y_velocity + gen_y_offset(self.left_paddle, center_y);
            return true;
        } else if self.x_velocity > 0.0 && hit_rect(self.right_paddle, center_x, center_y) {
            self.y_velocity = self.y_velocity + gen_y_offset(self.right_paddle, center_y);
            return true;
        }
        false
    }

    fn render(&mut self, _args: &RenderArgs, glyphs: &mut Glyphs, event: &piston_window::Event) {
        let ball = rectangle::centered_square(self.ball_x, self.ball_y, BALL_SIZE / 2.0);
        self.left_paddle = rectangle::centered([
            LEFT_PADDLE_POS,
            self.paddle_top[0],
            PADDLE_WIDTH / 2.0,
            PADDLE_HEIGHT / 2.0,
        ]);
        let left_paddle = self.left_paddle;
        self.right_paddle = rectangle::centered([
            RIGHT_PADDLE_POS,
            self.paddle_top[1],
            PADDLE_WIDTH / 2.0,
            PADDLE_HEIGHT / 2.0,
        ]);
        let right_paddle = self.right_paddle;

        let _left_score = self.score[0].to_string();
        let _right_score = self.score[1].to_string();

        self.window.draw_2d(event, |c, g| {
            clear(BLACK, g);

            // dots
            for i in 1..20 {
                let strip_y = (HEIGHT / BALL_SIZE) * i as f64;
                let strip = rectangle::centered_square(CENTER, strip_y, BALL_SIZE / 2.0);
                rectangle(GRAY, strip, c.transform, g);
            }
            rectangle(WHITE, ball, c.transform, g);
            rectangle(WHITE, left_paddle, c.transform, g);
            rectangle(WHITE, right_paddle, c.transform, g);
            // Draw the score
            let left_score_pos = c.transform.trans(cell_pos(4.0), 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_left_score, glyphs, &c.draw_state, left_score_pos, g)
                .unwrap();
            let right_score_pos = c.transform.trans(cell_pos(6.0), 100.0);
            text::Text::new_color(WHITE, FONTSIZE)
                .draw(&_right_score, glyphs, &c.draw_state, right_score_pos, g)
                .unwrap();
        });
    }

    fn reset_ball(&mut self, right: bool) {
        let mut rng = rand::thread_rng();
        self.ball_x = if right { WIDTH } else { 0.0 };
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
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.set_pressed(key);
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                self.set_released(key);
            }
        }
    }

    fn set_released(&mut self, _key: Key) {
        match _key {
            Key::W => self.pressed_keys[0] = false,
            Key::S => self.pressed_keys[1] = false,
            Key::I => self.pressed_keys[2] = false,
            Key::K => self.pressed_keys[3] = false,
            _ => {}
        };
    }
    fn set_pressed(&mut self, _key: Key) {
        match _key {
            Key::W => self.pressed_keys[0] = true,
            Key::S => self.pressed_keys[1] = true,
            Key::I => self.pressed_keys[2] = true,
            Key::K => self.pressed_keys[3] = true,
            _ => {}
        };
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // move ball
        self.ball_x = self.ball_x + self.x_velocity;
        self.ball_y = self.ball_y + self.y_velocity;
        //println!("{:?}", self.pressed_keys);
        // Point for left side
        if self.ball_x > WIDTH {
            self.score[0] = self.score[0] + 1;
            if self.score[0] > WINNING_SCORE {
                self.reset_game();
                return;
            }
            self.x_velocity = 0.0 - self.x_velocity;
            self.reset_ball(true);
            return;
        }
        // Point for right side;
        else if self.ball_x < 0.0 && self.x_velocity < 0.0 {
            self.score[1] = self.score[1] + 1;
            if self.score[1] > WINNING_SCORE {
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

        if self.hit_paddle() {
            self.x_velocity = 0.0 - self.x_velocity;
        }
        self.update_paddles();
    }
    fn update_paddles(&mut self) {
        if self.pressed_keys[0] && self.paddle_top[0] > 0.0 {
            self.paddle_top[0] -= PADDLE_VELOCITY
        }
        if self.pressed_keys[1] && self.paddle_top[0] < HEIGHT {
            self.paddle_top[0] += PADDLE_VELOCITY
        }
        if self.pressed_keys[2] && self.paddle_top[1] > 0.0 {
            self.paddle_top[1] -= PADDLE_VELOCITY
        }
        if self.pressed_keys[3] && self.paddle_top[1] < HEIGHT {
            self.paddle_top[1] += PADDLE_VELOCITY
        }
    }
}

fn cell_pos(_pos: f64) -> f64 {
    (WIDTH / CELL_COUNT) * _pos
}

pub fn hit_rect(_rect: [f64; 4], _x: f64, _y: f64) -> bool {
    if _x >= _rect[0] && _x <= _rect[0] + _rect[2] && _y >= _rect[1] && _y <= _rect[1] + _rect[3] {
        return true;
    }
    false
}

pub fn gen_y_offset(_rect: [f64; 4], _y: f64) -> f64 {
    (((_y - _rect[1]) / _rect[3]) * 4.0) - 2.0
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
    let mut pong = Pong::new(window);
    pong.run();
}

#[cfg(test)]
mod tests {
    use crate::hit_rect;
    #[test]
    fn test_hit_rect() {
        assert!(hit_rect([10.0, 10.0, 10.0, 10.0], 15.0, 15.0));
    }
    #[test]
    fn test_miss_rect() {
        assert!(!hit_rect([10.0, 10.0, 10.0, 10.0], 25.0, 25.0));
    }

    // Test y acceleration offset generation.
    use crate::gen_y_offset;
    #[test]
    fn test_no_offset() {
        assert_eq!(gen_y_offset([10.0, 10.0, 10.0, 10.0], 15.0), 0.0);
    }
    #[test]
    fn test_one_offset() {
        assert_eq!(gen_y_offset([10.0, 10.0, 10.0, 10.0], 10.0), -2.0);
        assert_eq!(gen_y_offset([10.0, 10.0, 10.0, 10.0], 20.0), 2.0);
    }
}
