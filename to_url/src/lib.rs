pub fn to_url(s: &str) -> String {
    s.chars().map(|a| if a.is_whitespace(){
        "%20".to_string()
    } else {
        a.to_string()
    }).collect()
}