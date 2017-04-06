//! San Diego Rusty Pong
//!
//! An example Pong implementation using the glium crate for the San Diego Rust group.

#[macro_use]
extern crate glium;
extern crate rand;

mod ball;
mod error;
mod paddle;
mod rectangle;

pub use paddle::Paddle;
pub use ball::Ball;
pub use rectangle::Rectangle;
use glium::glutin::VirtualKeyCode;
use std::collections::HashSet;

pub type Result<T> = std::result::Result<T, error::Error>;

/// The entry point for the game.
fn main() {
    // Create and run the game.
    // This will block until the game exits or an error occurs.
    let result = Game::run();

    // If an error occurred, print out the error and exit with an error code.
    if let Err(error) = result {
        use std::io::{stderr, Write};
        writeln!(&mut stderr(), "Error:\n{}", error).unwrap();
        std::process::exit(1);
    }
}

const GAME_WIDTH: u32 = 1280;
const GAME_HEIGHT: u32 = 720;
const GAME_FRAMERATE: f32 = 60.0;
const SCORE_TO_WIN: u32 = 10;

/// The controller for the game.
pub struct Game {
    display: glium::backend::glutin_backend::GlutinFacade,
    shader_program: glium::Program,
    rect_vertex_buffer: glium::VertexBuffer<Vertex>,

    width: f32,
    height: f32,
    frame_rate: f32,

    pressed_keys: HashSet<VirtualKeyCode>,

    left_paddle: Paddle,
    right_paddle: Paddle,

    ball: Ball,
}

impl Game {
    /// Creates and runs the game.
    /// This functions runs until the game exits or an error occurs.
    pub fn run() -> Result<()> {
        let mut game = Game::new()?;
        game.run_game_loop()?;
        Ok(())
    }

    /// Initializes the game.
    fn new() -> Result<Game> {
        // Create a window using glutin.
        use glium::DisplayBuild;
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(GAME_WIDTH, GAME_HEIGHT)
            .with_title("San Diego Rusty Pong")
            .build_glium()
            .unwrap();

        // Lock the cursor to the window.
        if let Some(window) = display.get_window() {
            window.set_cursor_state(glium::glutin::CursorState::Grab).unwrap_or(());
        }

        // Load the shader for drawing rectangles.
        let shader_program = Game::create_shader_program(&display)?;

        // Create the vertex buffer for a unit square.
        let rect_vertices = vec![
            Vertex { position: [0.0, 0.0] },
            Vertex { position: [0.0, 1.0] },
            Vertex { position: [1.0, 1.0] },

            Vertex { position: [0.0, 0.0] },
            Vertex { position: [1.0, 1.0] },
            Vertex { position: [1.0, 0.0] },
        ];
        let rect_vertex_buffer = glium::VertexBuffer::new(&display, &rect_vertices)?;

        // Initialize all game objects.
        let width = GAME_WIDTH as f32;
        let height = GAME_HEIGHT as f32;
        Ok(Game {
            display: display,
            shader_program: shader_program,
            rect_vertex_buffer: rect_vertex_buffer,

            width: width,
            height: height,
            frame_rate: GAME_FRAMERATE as f32,

            pressed_keys: HashSet::new(),

            left_paddle: Paddle::new(25.0, height / 2.0),
            right_paddle: Paddle::new(width - 25.0, height / 2.0),
            
            ball: Ball::new(width / 2.0, height / 2.0),
        })
    }

    /// Load and compile the shaders from the source files.
    /// The shader renders solidly filled polygons.
    fn create_shader_program(display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::Program> {
        use std::fs::File;
        use std::io::Read;

        let mut vertex_shader_file = File::open("shaders/vertex.glsl")?;
        let mut vertex_shader_src = String::new();
        vertex_shader_file.read_to_string(&mut vertex_shader_src)?;

        let mut fragment_shader_file = File::open("shaders/fragment.glsl")?;
        let mut fragment_shader_src = String::new();
        fragment_shader_file.read_to_string(&mut fragment_shader_src)?;

        let program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)?;

        Ok(program)
    }

    /// The game loop.
    /// Each iteration through the loop handles any window events, reads user input, 
    /// updates the game state, and renders a frame.
    /// This loop runs until the user requests an exit, or an error occurs.
    fn run_game_loop(&mut self) -> Result<()> {
        loop {
            let exit = self.poll_events();
            if exit {
                break;
            }

            // Update the game state.
            let frame_time = 1.0 / self.frame_rate;
            let params = UpdateParams {
                dt: frame_time,
                game_width: self.width,
                game_height: self.height,
            };
            self.update(&params);

            // Draw the frame.
            self.render()?;

            // Sleep until the next frame.
            use std::thread;
            use std::time::Duration;
            let sleep_time = Duration::from_millis((1000.0 * frame_time) as u64);
            thread::sleep(sleep_time);
        }

        // Game finished successfully.
        return Ok(());
    }

    /// Updates the game state.
    /// `dt` represents delta time, the amount of time that the game will be advanced.
    fn update(&mut self, params: &UpdateParams) {
        if !self.has_winner()
        {
            self.left_paddle.update(&params, &self.pressed_keys);
            self.right_paddle.update(&params, &self.pressed_keys);
            self.ball.update(&params, &mut self.left_paddle, &mut self.right_paddle);
        }
    }

    /// The winner is the first player to reach 10 points.
    fn has_winner(&mut self) -> bool {
        self.left_paddle.score >= SCORE_TO_WIN || self.right_paddle.score >= SCORE_TO_WIN
    }

    /// Renders the current game state.
    fn render(&mut self) -> Result<()> {
        // Ask glium for the buffer to draw to.
        use glium::Surface;
        let mut frame = self.display.draw();
        
        // Clear the screen.
        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        // Draw the various UI elements.
        self.draw_net(&mut frame)?;
        self.draw_score(&mut frame, self.left_paddle.score, self.width * 0.25, 10.0)?;
        self.draw_score(&mut frame, self.right_paddle.score, self.width * 0.75, 10.0)?;

        // Draw the player paddles.
        self.left_paddle.render(&self, &mut frame)?;
        self.right_paddle.render(&self, &mut frame)?;

        // Draw the ball.
        self.ball.render(&self, &mut frame)?;

        // Finish drawing and present the buffer.
        Ok(frame.finish()?)
    }

    /// Draws a dotted line in the middle of the screen.
    fn draw_net(&self, frame: &mut glium::Frame) -> Result<()> {
        const NET_WIDTH: f32 = 8.0;
        const NET_SEGMENT_HEIGHT: f32 = 50.0;
        let mut rect = Rectangle {
            x: (self.width - NET_WIDTH) / 2.0,
            y: 0.0,
            width: NET_WIDTH,
            height: NET_SEGMENT_HEIGHT
        };

        while rect.y < self.height {
            self.draw_rectangle(frame, rect, [0.1, 0.1, 0.1, 1.0])?;
            rect.y += NET_SEGMENT_HEIGHT * 1.5;
        }
        Ok(())
    }

    /// Draws a series of dots representing the score for a player.
    fn draw_score(&self, frame: &mut glium::Frame, score: u32, x: f32, y: f32) -> Result<()> {
        const ROW_LENGTH: u32 = 5;
        let mut rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0
        };
        // Draw winning score in red.
        let color = if score < SCORE_TO_WIN { [0.2, 0.2, 0.2, 1.0] } else {[1.0, 0.2, 0.2, 1.0] };
        for i in 0..score {
            let column = (i % ROW_LENGTH) as f32;
            let row = (i / ROW_LENGTH) as f32;
            rect.x = x + 8.0 * column;
            rect.y = y + 8.0 * row;
            self.draw_rectangle(frame, rect, color)?;
        }
        Ok(())
    }

    /// Handles any new window or UI events.
    /// This includes window resizing, keyboard presses, mouse input, etc.
    /// This must be called once per frame to keep the app responsive.
    /// Returns `true` if the user requested to exit the game.
    fn poll_events(&mut self) -> bool {
        use glium::glutin::{ElementState, Event};

        for event in self.display.poll_events() {
            match event {
                // Window closed by the user.
                Event::Closed => return true,
                
                // User pressed Escape to close the game.
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    return true;
                },

                // Keyboard input.
                Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
                    self.pressed_keys.insert(key);
                },

                Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                    self.pressed_keys.remove(&key);
                },

                // TODO: Handle mouse/touch events.

                // Other events are unhandled.
                _ => (),
            }
        }

        false
    }

    /// Draws a rectangle onto the given frame buffer.
    /// x and y are in game coordinates (1280x720) with (0, 0) at the top left of the frame.
    pub fn draw_rectangle(&self, frame: &mut glium::Frame, rect: Rectangle, color: [f32; 4]) -> Result<()> {
        use glium::Surface;
        
        let (frame_width, frame_height) = (frame.get_dimensions().0 as f32, frame.get_dimensions().1 as f32);

        // Matrix to transform the 1x1 square at (0, 0) into a (width, height) square at (x, y).
        let transform: [[f32; 4]; 4] = [
            [rect.width, 0.0, 0.0, 0.0],
            [0.0, rect.height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [rect.x, rect.y, 0.0, 1.0],
        ];

        // Matrix to project square from 2D screen coordinates into OpenGL device coordinates.
        let scale = f32::min(frame_width  / self.width, frame_height / self.height);
        let shift_x = 1.0 - self.width * scale / frame_width;
        let shift_y = self.height * scale / frame_height - 1.0;
        let projection: [[f32; 4]; 4] = [
            [2.0 * scale / frame_width, 0.0, 0.0, 0.0],
            [0.0, -2.0 * scale / frame_height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-1.0 + shift_x, 1.0 + shift_y, 0.0, 1.0],
        ];

        // Render the quad using the calculated transform.
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        Ok(
            frame.draw(
                &self.rect_vertex_buffer,
                &indices,
                &self.shader_program,
                &uniform! { color: color, transform: transform, projection: projection },
                &Default::default())?
        )
    }
}

/// Information about the current game frame.
/// Gets passed to each game object during update.
#[derive(Clone, Debug)]
pub struct UpdateParams {
    pub dt: f32,              // The amount of time in seconds to advance the simulation.
    pub game_width: f32,
    pub game_height: f32,
}


/// The per-vertex data for our triangles.
/// `postion` is the only vertex attribute because we are only rendering solidly filled
/// 2d polygons.
#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
}
// This is a magic glium macro to implement the required `Vertex` trait
// for our vertex structure. This trait builds the vertex format information
// required by glium and OpenGL.
//
// For more information, see:
// https://tomaka.github.io/glium/glium/macro.implement_vertex!.html
// https://tomaka.github.io/glium/glium/vertex/trait.Vertex.html
implement_vertex!(Vertex, position);
