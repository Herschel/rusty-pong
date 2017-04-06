//! Represents a player's paddle in a game of Pong.

use {Game, Rectangle, Result, UpdateParams};
use glium::Frame;
use glium::glutin::VirtualKeyCode;
use std::collections::HashSet;

const KEYBOARD_SPEED: f32 = 500.0;
const WIDTH: f32 = 20.0;
const HEIGHT: f32 = 100.0;

/// Each paddle has a position and a score.
#[derive(Clone, Debug)]
pub struct Paddle {
    pub bounds: Rectangle,
    pub score: u32,
}

impl Paddle {
    /// Creates a paddle at the given position.
    /// The paddle can only move vertically.
    pub fn new(x: f32, y: f32) -> Paddle {
        Paddle {
            bounds: Rectangle {
                x: x - WIDTH / 2.0,
                y: y - HEIGHT / 2.0,
                width: WIDTH,
                height: HEIGHT,
            },
            score: 0,
        }
    }

    /// Updates the state of the paddle based on player input.
    pub fn update(&mut self, params: &UpdateParams, pressed_keys: &HashSet<VirtualKeyCode>) {
        /// Move the paddle if a particular key is pressed.
        // TODO: Allow the left and right paddles to be controlled via different keys/mouse.
        let mut vy = 0.0;
        if pressed_keys.contains(&VirtualKeyCode::Up) {
            vy -= KEYBOARD_SPEED;
        }
        if pressed_keys.contains(&VirtualKeyCode::Down) {
            vy += KEYBOARD_SPEED;
        }

        self.bounds.y += vy * params.dt; 

        // Clamp the paddle position on screen.
        if self.bounds.y < 0.0 {
            self.bounds.y = 0.0;
        } else if self.bounds.y + self.bounds.height > params.game_height {
            self.bounds.y = params.game_height - self.bounds.height;
        }
    }

    /// Draws the paddle on the screen.
    pub fn render(&self, game: &Game, frame: &mut Frame) -> Result<()> {
        game.draw_rectangle(frame, self.bounds, [1.0, 1.0, 1.0, 1.0])
    }
}
