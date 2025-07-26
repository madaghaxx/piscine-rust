pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '-' => {
                result.pop();
            }
            '+' => {
                // mrdatni
            }
            c => {
                result.push(c);
            }
        }
        i += 1;
    }
    *s = result;
}

// pub fn do_operations(v: &mut [String]) {}
