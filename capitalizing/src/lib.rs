pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.collect::<String>(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();
    
    for c in input.chars() {
        if c.is_whitespace() {
            if !word.is_empty() {
                result.push_str(&capitalize_first(&word));
                word.clear();
            }
            result.push(c);
        } else {
            word.push(c);
        }
    }
    
    if !word.is_empty() {
        result.push_str(&capitalize_first(&word));
    }
    
    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}
