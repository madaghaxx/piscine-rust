pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut i = 0;
    for character in s.chars() {
        if character == '-' {
            result.pop();
        } else if character == '+' {
            i += 1;
        } else {
            if i != 0 {
                i -= 1;
                continue;
            }
            result.push(character);
        }
    }

    *s = result;
}
pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        let mut chars = s.chars();
        let mut lwl = String::new();
        let mut zawj = String::new();
        let mut op = None;
        while let Some(c) = chars.next() {
            if c == '+' || c == '-' {
                op = Some(c);
                break;
            }
            lwl.push(c);
        }
        for c in chars {
            zawj.push(c);
        }

        if let (Ok(a), Some(op), Ok(b)) = (lwl.parse::<i32>(), op, zawj.parse::<i32>()) {
            let res = match op {
                '+' => a + b,
                '-' => a - b,
                _ => {
                    continue;
                }
            };
            *s = res.to_string();
        }
    }
}
