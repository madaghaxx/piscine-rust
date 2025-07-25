pub fn first_subword(s: String) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c == '_' || c == ' ' {
            break;
        } else {
            res.push(c);
        }
    }
    res
}
