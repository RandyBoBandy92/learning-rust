use std::f64::consts::PI;

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        return PI * self.radius.powf(2.0);
    }
    pub fn get_diameter(&self) -> f64 {
        return self.radius * 2.0;
    }
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        return self.width * self.height;
    }
}
