use std::collections::HashMap;

fn word_frequency_calculator(string: &str) -> HashMap<&str, usize> {
    let mut map: HashMap<&str, usize> = HashMap::new();
    for word in string.split_whitespace() {
        let count: &mut usize = map.entry(word).or_insert(0);
        *count += 1;
    }
    return map;
}

fn main() {
    let string: &str = "hello world hello world hello world banana";
    let map: HashMap<&str, usize> = word_frequency_calculator(string);
    println!("{:?}", map);
}
