pub fn is_pangram(s: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for letter in alphabet.chars() {
        if !s.to_lowercase().contains(letter) {
            return false;
        }
    }
    true
}