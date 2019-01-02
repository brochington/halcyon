#[derive(Debug, Clone)]
pub enum ColorModels {
  RGB,
  RGBA,
}

#[derive(Debug, Clone)]
pub struct Color {
  pub model: ColorModels,
  pub color: Vec<f64>,
}

impl Color {
  pub fn new() -> Color {
    Color {
      model: ColorModels::RGB,
      color: vec![1.0, 1.0, 1.0]
    }
  }

  pub fn new_rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
    Color {
      model: ColorModels::RGBA,
      color: vec![r, g, b, a]
    }
  }

  pub fn get_rgb(&self) -> [f64; 3] {
    match self.model {
      ColorModels::RGB => [ self.color[0], self.color[1], self.color[2] ],
      ColorModels::RGBA => [ self.color[0], self.color[1], self.color[2] ]
    }
  }
}

pub fn set_rgb(mut color: Color, r: f64, g: f64, b: f64 ) -> Color {
  match color.model {
    ColorModels::RGB => {
      color.color = vec![r, g, b];
      color
    },
    ColorModels::RGBA => {
      color.color = vec![r, g, b, 1.0];
      color
    }
  }
}

pub fn get_rgb(color: Color) -> [f64; 3] {
  match color.model {
    ColorModels::RGB => [ color.color[0], color.color[1], color.color[2] ],
    ColorModels::RGBA => [ color.color[0], color.color[1], color.color[2] ]
  }
}

