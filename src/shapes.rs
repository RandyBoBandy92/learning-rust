use std::f64::consts::PI;

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub fn calculate_circle_area(c: &Circle) -> f64 {
    return PI * c.radius.powf(2.0);
}

pub fn calculate_rectangle_area(r: &Rectangle) -> f64 {
    return r.width * r.height;
}
