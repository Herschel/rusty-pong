//! The custom error type for the game.
//!
//! For more info, see the Error Handling section of the Rust book:
//! https://doc.rust-lang.org/book/error-handling.html

use glium;
use std::error::Error as StdError;
use std::fmt;
use std::io;

/// The custom error type for Rusty Pong.
/// Wraps any IO or glium errors.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ShaderProgramError(glium::program::ProgramCreationError),
    SwapBuffersError(glium::SwapBuffersError),
    BufferCreationError(glium::vertex::BufferCreationError),
    DrawError(glium::DrawError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<glium::program::ProgramCreationError> for Error {
    fn from(err: glium::program::ProgramCreationError) -> Error {
        Error::ShaderProgramError(err)
    }
}

impl From<glium::SwapBuffersError> for Error {
    fn from(err: glium::SwapBuffersError) -> Error {
        Error::SwapBuffersError(err)
    }
}

impl From<glium::vertex::BufferCreationError> for Error {
    fn from(err: glium::vertex::BufferCreationError) -> Error {
        Error::BufferCreationError(err)
    }
}

impl From<glium::DrawError> for Error {
    fn from(err: glium::DrawError) -> Error {
        Error::DrawError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::ShaderProgramError(ref err) => err.fmt(f),
            Error::SwapBuffersError(ref err) => err.fmt(f),
            Error::BufferCreationError(ref err) => err.fmt(f),
            Error::DrawError(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::ShaderProgramError(ref err) => err.description(),
            Error::SwapBuffersError(ref err) => err.description(),
            Error::BufferCreationError(ref err) => err.description(),
            Error::DrawError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ShaderProgramError(ref err) => Some(err),
            Error::SwapBuffersError(ref err) => Some(err),
            Error::BufferCreationError(ref err) => Some(err),
            Error::DrawError(ref err) => Some(err),
        }
    }
}