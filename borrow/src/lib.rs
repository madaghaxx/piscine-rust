pub fn str_len(s: &str) -> usize {
    let arr = s.clone().chars().collect::<Vec<char>>();
    arr.len()
}
