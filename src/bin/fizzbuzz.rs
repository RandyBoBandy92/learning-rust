// Write a program that uses a for loop to iterate over the numbers 1 to 100,
// printing "Fizz" if the number is divisible by 3, "Buzz" if it's divisible by 5, and "FizzBuzz"
//  if it's divisible by both 3 and 5. If the number is not divisible by either 3 or 5, print the number itself.

fn main() {
    for i in 1..=100 {
        let divisible_by_3 = i % 3 == 0;
        let divisible_by_5 = i % 5 == 0;
        let divisible_by_both = divisible_by_3 && divisible_by_5;
        if divisible_by_both {
            println!("FizzBuzz")
        } else if divisible_by_3 {
            println!("Fizz")
        } else if divisible_by_5 {
            println!("Buzz")
        } else {
            println!("{}", i);
        }
    }
}
