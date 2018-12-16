use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn clone(self) -> Vec2 {
        Vec2 {
          x: self.x,
          y: self.y,  
        }
    }

    pub fn from(x: f64, y: f64) -> Vec2 {
        Vec2 {
            x,
            y,
        }
    }
    
    // AKA "length" of vector
    pub fn magnitude(&self) -> f64 {
        // ||V|| = sqrt(x^2 + y^2)
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(self, q: Vec2) -> f64 {
        // p . q = p1 * q1 + p2 * q2
        self.x * q.x + self.y * q.y
    }

    pub fn scale(self, scale: f64) -> Vec2 {
        Vec2 {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

// Operator overloads
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}