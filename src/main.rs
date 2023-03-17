mod colors;
mod foo;
mod shapes;
mod strings;
use colors::{print_color_info, Colors};

fn main() {
    // println!("Hello, world!");
    // foo::bar();

    let s = "Hello, world!";
    println!("Length of string: {}", strings::get_length_of_string(s));
    println!(
        "Number of words in string: {}",
        strings::get_num_of_word_in_string(s)
    );

    let c = shapes::Circle { radius: 5.0 };
    let r = shapes::Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("Area of circle: {}", c.area());
    let diameter = c.get_diameter();
    if diameter > 10.0 {
        println!("Diameter is greater than 10.0");
    } else {
        println!("Diameter is less than 10.0");
    }
    println!("Area of rectangle: {}", r.area());

    print_color_info(Colors::Red(255));
}
