fn calculate(x: i32, y: i32, operator: char) -> Result<i32, String> {
    // Result is an enum with two variants: Ok and Err
    // Kind of like a JavaScript Promise
    match operator {
        // match is like a switch statement
        '+' => Ok(x + y),
        '-' => Ok(x - y),
        '*' => Ok(x * y),
        '/' => {
            if y == 0 || x == 0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(x / y)
            }
        }
        // _ is a catch-all, like a default case
        _ => Err(format!("Unknown operator: {}", operator)),
    }
}

fn main() {
    let test_cases = vec![
        (1, 2, '+'),
        (1, 2, '-'),
        (1, 2, '*'),
        (1, 2, '/'),
        (1, 2, 'a'),
        (1, 0, '/'),
    ];

    // writing it this way "consumes" the values in the vector
    // meaning I took "ownership" of the values in the vector
    // and I can't use them again
    // for (x, y, operator) in test_cases {
    //     match calculate(x, y, operator) {
    //         Ok(result) => println!("{} {} {} = {}", x, operator, y, result),
    //         Err(error) => println!("{} {} {} = {}", x, operator, y, error),
    //     }
    // }

    // if I want to use the values again, I need to clone them using .iter()
    for (x, y, operator) in test_cases.iter() {
        // In Rust, the & prefix is used to create a reference to a value,
        // and the * prefix is used to dereference a reference, giving you access to the actual value it points to.
        match calculate(*x, *y, *operator) {
            // Because Result is an enum, I can use the match keyword to destructure it
            Ok(result) => println!("{} {} {} = {}", x, operator, y, result),
            Err(error) => println!("ERROR: {} {} {} = {}", x, operator, y, error),
        }
    }
}
