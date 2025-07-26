pub fn char_length(s: &str) -> usize {
    let arr = s.clone().chars().collect::<Vec<char>>();
    arr.len()
}
