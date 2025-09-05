pub fn first_subword(mut s: String) -> String {
    if let Some(pos) = s.chars().skip(1).position(|ch| ch.is_uppercase() || ch == '_') {
        s.truncate(pos + 1);
    }
    s
}
