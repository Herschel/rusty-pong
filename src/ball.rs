//! Represents the ball in a game of Pong.

use {Game, Paddle, Rectangle, Result, UpdateParams};
use glium::Frame;

const WIDTH: f32 = 15.0;
const HEIGHT: f32 = 15.0;
const BALL_BOUNCE_SPEEDUP: f32 = 1.15;
const BALL_STARTING_SPEED: f32 = 500.0;

// The ball has a speed and moves once per frame.
#[derive(Clone, Debug)]
pub struct Ball {
    bounds: Rectangle,
    vx: f32,
    vy: f32,
    start_timer: f32,
}

impl Ball {
    // Creates a new ball at the given position.
    pub fn new(x: f32, y: f32) -> Ball {
        let mut ball = Ball {
            vx: 0.0,
            vy: 0.0,
            start_timer: 0.0,
            bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: WIDTH,
                height: HEIGHT,
            },
        };
        ball.reset(x, y);
        ball
    }

    /// Resets the ball back to the given position.
    /// The ball will stay in place for a moment before moving.
    fn reset(&mut self, x: f32, y: f32) {
        self.bounds.x = x - self.bounds.width / 2.0;
        self.bounds.y = y - self.bounds.height / 2.0;
        
        use rand::{self, Rng};
        let mut rng = rand::thread_rng();

        // Generate a random velocity towards one player.
        self.vx = if rng.gen() { BALL_STARTING_SPEED } else { -BALL_STARTING_SPEED };
        self.vy = rng.gen_range( -BALL_STARTING_SPEED, BALL_STARTING_SPEED );
        self.start_timer = 1.0;
    }

    /// Updates the position of the ball and checks for collisions.
    pub fn update(&mut self, params: &UpdateParams, left_paddle: &mut Paddle, right_paddle: &mut Paddle) {
        // The ball stays still until a timer elapses.
        if self.start_timer > 0.0 {
            self.start_timer -= params.dt;
        } else {
            self.bounds.x += self.vx * params.dt;
            self.bounds.y += self.vy * params.dt;
        }

        // Check collision.
        self.check_paddle_collision(left_paddle);
        self.check_paddle_collision(right_paddle);
        self.check_wall_collision(params);
        self.check_goal(params, left_paddle, right_paddle);
    }

    /// Draws the paddle on the screen.
    pub fn render(&self, game: &Game, frame: &mut Frame) -> Result<()> {
        game.draw_rectangle(frame, self.bounds, [1.0, 1.0, 1.0, 1.0])
    }

    /// Handles collision between the ball and a paddle.
    fn check_paddle_collision(&mut self, paddle: &Paddle) {
        if paddle.bounds.intersects(self.bounds) {
            // Snap the edge of the ball to the edge of the paddle.
            self.bounds.x = if self.vx < 0.0 { paddle.bounds.x + paddle.bounds.width } else { paddle.bounds.x - self.bounds.width };
            // Reflect the ball the opposite direction.
            self.vx = -BALL_BOUNCE_SPEEDUP * self.vx;
            // Adjust the vertical velocity of the ball based on where it hits the paddle.
            let dy = self.bounds.y - paddle.bounds.y;
            self.vy = dy * 10.0;
        }
    }

    /// Handles collision between the ball and the top or bottom of the screen.
    fn check_wall_collision(&mut self, params: &UpdateParams) {
        if self.bounds.y <= 0.0 || self.bounds.y + self.bounds.height >= params.game_height {
            // Flip vertically.
            self.vy = -self.vy;
        }
    }

    /// Handles collision between the ball and the left or right edge of the screen.
    fn check_goal(&mut self, params: &UpdateParams, left_paddle: &mut Paddle, right_paddle: &mut Paddle) {
        // Check for goal.
        if self.bounds.x <= 0.0 {
            right_paddle.score += 1;
            self.reset(params.game_width / 2.0, params.game_height / 2.0);
        } else if self.bounds.x + self.bounds.width >= params.game_width {
            left_paddle.score += 1;
            self.reset(params.game_width / 2.0, params.game_height / 2.0);
        }
    }
}
