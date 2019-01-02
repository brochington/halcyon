pub mod color_texture;
pub mod image_texture;

pub use self::color_texture::ColorTexture;
pub use self::image_texture::ImageTexture;

#[derive(Debug, Clone)]
pub enum Textures {
  ColorTexture(ColorTexture),
  ImageTexture(ImageTexture)
}