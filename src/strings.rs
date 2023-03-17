pub fn get_length_of_string(s: &str) -> usize {
    s.len()
}

pub fn get_num_of_word_in_string(s: &str) -> usize {
    s.split_whitespace().count()
}
