pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::from(""),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut word_start = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            word_start = true;
        } else {
            if word_start {
                for up in c.to_uppercase() {
                    result.push(up);
                }
                word_start = false;
            } else {
                for low in c.to_lowercase() {
                    result.push(low);
                }
            }
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
