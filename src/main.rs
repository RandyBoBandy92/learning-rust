mod foo;
mod shapes;
mod strings;

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
    println!("Area of circle: {}", shapes::calculate_circle_area(&c));
    println!(
        "Area of rectangle: {}",
        shapes::calculate_rectangle_area(&r)
    );
}
