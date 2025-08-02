pub fn rotate(input: &str, key: i8) -> String {
    let mut coded = "".to_string();
    if key == 0 {
        return (&input).to_string();
    }
    for c in input.chars() {
        if c.is_alphabetic() {
            let base = if c.is_lowercase() { b'a' } else { b'A' };
            let shifted = ((((c as u8) - base) as i8) + key).rem_euclid(26) as u8;
            coded += &char::from(base + shifted).to_string();
        } else {
            coded += &c.to_string();
        }
    }
    coded
}
