pub fn is_pangram(s: &str) -> bool {
    let s = s.to_lowercase();
    for c in 'a'..='z' {
        if !s.contains(c) {
            return false;
        }
    }
    true
}