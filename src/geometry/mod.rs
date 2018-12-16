pub mod triangle;
pub mod square;

pub use self::triangle::Triangle2D;
pub use self::square::Square2D;

#[derive(Debug, Clone)]
pub enum Geometries {
  Triangle2D(Triangle2D),
  Square2D(Square2D),
}