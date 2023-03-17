fn calculate(x: i32, y: i32, operator: char) -> Result<i32, String> {
    match operator {
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
        _ => Err(format!("Unknown operator: {}", operator)),
    }
}

fn main() {
    // lets test the calculate function
    let result1 = calculate(10, 5, '+');
    let result2 = calculate(10, 5, '-');
    // try different values for x, y and operator
    let result3 = calculate(53, 5, '*');
    let result4 = calculate(10, 5, '/');
    // now lets attempt to divide by zero
    let result5 = calculate(10, 0, '/');
}
