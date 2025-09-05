pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.into_iter().map(|name| {
        let mut result = String::new();
        let mut first = true;
        
        for word in name.split_whitespace() {
            if let Some(ch) = word.chars().next() {
                if !first {
                    result.push(' ');
                }
                result.push(ch);
                result.push('.');
                first = false;
            }
        }
        result
    }).collect()
}