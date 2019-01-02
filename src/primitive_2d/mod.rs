pub mod triangle;
pub mod rectangle;

pub use self::triangle::Triangle2D;
pub use self::rectangle::Rectangle2D;

pub trait Primitive2D {
}

#[derive(Debug, Clone)]
pub enum Primitives2D {
  Triangle2D(Triangle2D),
  Rectangle2D(Rectangle2D)
}