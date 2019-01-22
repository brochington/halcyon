pub trait Color<T> {
  fn new() -> Self;

  fn new_rgb() -> Self;

  fn new_rgba() -> Self;

  fn from_rgb(r: T, g: T, b: T) -> Self;

  fn from_rgba(r: T, g: T, b: T, a: T) -> Self;

  fn get_rgb(&self) -> [T; 3];

  fn get_rgba(&self) -> [T; 4];
}

#[derive(Debug, Clone)]
pub enum ColorSpaces {
  RGB,
  RGBA,
}

#[derive(Debug)]
pub enum Colors {
  ColorU8,
  ColorF32
}

#[derive(Clone, Debug)]
pub struct ColorU8 {
  pub space: ColorSpaces,
  data: Vec<u8>
}

impl Color<u8> for ColorU8 {
  fn new() -> ColorU8 {
    ColorU8 {
      space: ColorSpaces::RGB,
      data: vec![0u8, 0, 0]
    }
  }

  fn new_rgb() -> ColorU8 {
    ColorU8 {
      space: ColorSpaces::RGB,
      data: vec![0u8, 0, 0]
    }
  }

  fn new_rgba() -> ColorU8 {
    ColorU8 {
      space: ColorSpaces::RGBA,
      data: vec![0u8, 0, 0, 0]
    }
  }

  fn from_rgb(r: u8, g: u8, b: u8) -> ColorU8 {
    ColorU8 {
      space: ColorSpaces::RGB,
      data: vec![r, g, b]
    }
  }

  fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorU8 {
    ColorU8 {
      space: ColorSpaces::RGBA,
      data: vec![r, g, b, a]
    }
  }

  fn get_rgb(&self) -> [u8; 3] {
    match self.space {
      ColorSpaces::RGB => {
        [
          self.data[0],
          self.data[1],
          self.data[2],
        ]
      },
      ColorSpaces::RGBA => {
        [
          self.data[0],
          self.data[1],
          self.data[2],
        ]
      }
    }
  }

  fn get_rgba(&self) -> [u8; 4] {
    match self.space {
      ColorSpaces::RGB => {
        [
          self.data[0],
          self.data[1],
          self.data[2],
          1
        ]
      },
      ColorSpaces::RGBA => {
        [
          self.data[0],
          self.data[1],
          self.data[2],
          self.data[3]
        ]
      }
    }
  }
}

#[derive(Clone, Debug)]
pub struct ColorF32 {
  pub space: ColorSpaces,
  data: Vec<f32>
}

impl Color<f32> for ColorF32 {
  fn new() -> ColorF32 {
    ColorF32 {
      space: ColorSpaces::RGB,
      data: vec![0.0f32, 0.0, 0.0]
    }
  }

  fn new_rgb() -> ColorF32 {
    ColorF32 {
      space: ColorSpaces::RGB,
      data: vec![0.0f32, 0.0, 0.0]
    }
  }

  fn new_rgba() -> ColorF32 {
    ColorF32 {
      space: ColorSpaces::RGBA,
      data: vec![0.0f32, 0.0, 0.0, 0.0]
    }
  }

  fn from_rgb(r: f32, g: f32, b: f32) -> ColorF32 {
    ColorF32 {
      space: ColorSpaces::RGB,
      data: vec![r, g, b]
    }
  }

  fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> ColorF32 {
    ColorF32 {
      space: ColorSpaces::RGBA,
      data: vec![r, g, b, a]
    }
  }

  fn get_rgb(&self) -> [f32; 3] {
    match self.space {
      ColorSpaces::RGB => {
        [
         self.data[0],
         self.data[1],
         self.data[2] 
        ]
      },
      ColorSpaces::RGBA => {
        [
         self.data[0],
         self.data[1],
         self.data[2] 
        ]
      }
    }
  }

  fn get_rgba(&self) -> [f32; 4] {
    match self.space {
      ColorSpaces::RGB => {
        [
         self.data[0],
         self.data[1],
         self.data[2],
         1.0
        ]
      },
      ColorSpaces::RGBA => {
        [
         self.data[0],
         self.data[1],
         self.data[2],
         self.data[3]
        ]
      }
    }
  }
}