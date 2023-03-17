pub enum Colors {
    Red(u8),
    Green(u8),
    Blue(u8),
}

pub fn print_color_info(color: Colors) {
    match color {
        Colors::Red(r) => println!("Red: {}", r),
        Colors::Green(g) => println!("Green: {}", g),
        Colors::Blue(b) => println!("Blue: {}", b),
    }
}
