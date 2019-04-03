extern crate find_folder;
extern crate piston_window;
//extern crate rand;

use piston_window::*;
//use rand::Rng;

pub struct App {
    window: PistonWindow, // OpenGL drawing backend.
    ball_x: f64,
    ball_y: f64,
    current_width: f64,
    current_height: f64,
    left_paddle: f64,
    right_paddle: f64,
    x_velocity: f64,
    y_velocity: f64,
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
            left_paddle: 40.0,
            right_paddle: 40.0,
            x_velocity: 10.0,
            y_velocity: 10.0,
            score: [0, 0],
        }
    }

    fn render(&mut self, _args: &RenderArgs, glyphs: &mut Glyphs, event: &piston_window::Event) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
        const FONTSIZE: u32 = 48;

        const BALL_SIZE: f64 = 20.0;

        let center_x = _args.width / 2.0;
        let ball = rectangle::square(self.ball_x, self.ball_y, BALL_SIZE);
        let (paddle_width, paddle_height) = (_args.width / 50.0, _args.height / 10.0);
        let left_paddle_pos = _args.width / 20.0;
        let right_paddle_pos = left_paddle_pos * 19.0;
        let left_paddle = rectangle::rectangle_by_corners(
            left_paddle_pos,
            self.left_paddle,
            left_paddle_pos + paddle_width,
            self.left_paddle + paddle_height,
        );
        let right_paddle = rectangle::rectangle_by_corners(
            right_paddle_pos,
            self.right_paddle,
            right_paddle_pos + paddle_width,
            self.left_paddle + paddle_height,
        );

        let mut _left_score = self.score[0].to_string();
        let mut _right_score = self.score[1].to_string();
        self.current_height = _args.height;
        self.current_width = _args.width;

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
            //let text_image = Image::new_color(WHITE);
            // text_image.build
        });
    }

//    fn reset_ball(&mut.self, bool::right) {
//;        self.ball_x=0,self.ball_y=0;
//    }

 //   fn reset_game() {
  //      self.left_score=0;
   //     self.right_score=0;
 //       self.reset_ball(false);
        
//    }

    fn run(&mut self) {
        // Load font
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("Square.ttf");
        let factory = self.window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
        let mut events = Events::new(EventSettings::new());
//        self.reset_game();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r, &mut glyphs, &e);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // move ball
        self.ball_x = self.ball_x + self.x_velocity;
        self.ball_y = self.ball_y + self.y_velocity;
        if self.current_width > 0.0 && self.ball_x > self.current_width {
            self.score[0] = self.score[0] + 1;
            self.x_velocity=0.0-self.x_velocity;
        }
        if self.ball_x < 0.0 {
            self.score[1] = self.score[1] + 1;
            self.x_velocity=0.0-self.x_velocity;
        }
        if self.ball_y < 0.0 {
            self.y_velocity=0.0-self.y_velocity;
        }
        if self.current_height > 0.0 && self.ball_y > self.current_height {
            self.y_velocity=0.0-self.y_velocity;
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
