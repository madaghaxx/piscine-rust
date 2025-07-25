pub fn to_url(s: &str) -> String {
    s.chars()
        .map(|x| {
            if x == ' ' {
                "%20".to_string()
            } else {
                x.to_string()
            }
        })
        .collect::<String>()
}
