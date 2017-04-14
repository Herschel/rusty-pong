//! Represents a quad or axis-aligned bounding box.

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    /// Creates a new rectangle with the top-left corner at the specified position.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle { x: x, y: y, width: width, height: height }
    }

    /// Creates a new rectangle centered at the specified position.
    pub fn new_centered(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x: x - width / 2.0,
            y: y - height / 2.0,
            width: width,
            height: height
        }
    }

    /// Tests if `self` contains the given point.
    #[allow(dead_code)]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Tests if `self` intersects the given rectangle.
    pub fn intersects(&self, other: Rectangle) -> bool {
        self.x <= other.x + other.width && self.x + self.width >= other.x &&
        self.y <= other.y + other.height && self.y + self.height >= other.y
    }
}
