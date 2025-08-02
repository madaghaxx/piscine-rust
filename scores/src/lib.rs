use std::collections::HashMap;
pub fn score(s: &str) -> u64 {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut map = HashMap::new();
    for c in alphabet.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => map.insert(c, 1),
            'd' | 'g' => map.insert(c, 2),
            'b' | 'c' | 'm' | 'p' => map.insert(c, 3),
            'f' | 'h' | 'v' | 'w' | 'y' => map.insert(c, 4),
            'k' => map.insert(c, 5),
            'j' | 'x' => map.insert(c, 8),
            'q' | 'z' => map.insert(c, 10),
            _ => Some(0),
        }
    }
    let mut score = 0;
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            score += map.get(&c.to_ascii_lowercase()).unwrap_or(&0);
        }
    }
    score
}
