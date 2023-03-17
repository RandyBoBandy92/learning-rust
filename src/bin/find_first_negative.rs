// Implement a function find_first_negative that takes a slice of integers and returns an Option<i32>.
//  The function should return the first negative number in the slice or None if there are no
//  negative numbers. Test the function with different inputs and handle the result using a match expression.

fn find_first_negative(slice: &[i32]) -> Option<i32> {
    for &num in slice {
        if num < 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    // I make a vector of vectors because I want test data with varying lengths
    let test_cases = vec![
        vec![1, 2, 3, 4, 5],
        vec![-1, 2, 3, 4, 5],
        vec![1, 2, 3, -4, 5],
        vec![1, 2, 3, 4, -5, 7, 25],
    ];
    // I use .iter() to iterate over the values in the vector and return a reference to each value
    // I use .enumerate() to get the index of each value
    // then I pass &test_case with the & prefix to create a reference to the value
    // I don't have to use * to dereference the reference because I'm passing a reference to the value
    // and my function is expecting a reference to a slice of integers
    for (index, test_case) in test_cases.iter().enumerate() {
        match find_first_negative(&test_case) {
            Some(result) => println!(
                "The first negative number in Test Case # {} - {:?} is {}",
                index + 1,
                test_case,
                result
            ),
            None => println!(
                "There are no negative numbers in Test Case # {} - {:?}",
                index + 1,
                test_case
            ),
        }
    }
}
