use std::env;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, Read};
// I use self to get access to io::Error type

fn read_file_contents(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    // the ? operator will return the Err value from the current function for
    //  the caller to handle
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // we use ? twice because read_to_string returns a Result
    // and may also fail
    Ok(contents)
    // because this function returns a Result, we can use ? in the body
    // and Ok() here to return the value we want
    // kind of like Promise.resolve() in JS
}

// Create a program that demonstrates the use of the ? operator with the std::fs::File
//  and std::io::Read traits to read the contents of a file. The program should take the
//  file name as a command-line argument and print the contents of the file or an error message if the file cannot be read.

fn main() {
    println!("Current directory: {:?}", current_dir()); // Ok("/Users/Randy/Documents/temp/learning-rust")
    let file_name = "./src/bin/text.txt"; // i got an error here until I used a relative path
    let contents = read_file_contents(file_name);
    match contents {
        Ok(contents) => println!("Contents of {}: {}", file_name, contents),
        Err(e) => println!("Error reading {}: {}", file_name, e),
    }
    // now lets try to read a file that doesn't exist
    let file_name = "does_not_exist.txt";
    let contents = read_file_contents(file_name);
    match contents {
        Ok(contents) => println!("Contents of {}: {}", file_name, contents),
        Err(e) => println!("Error reading {}: {}", file_name, e),
    }
}
