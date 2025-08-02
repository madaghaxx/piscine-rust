pub fn pig_latin(text: &str) -> String {
    let first_char = text.chars().next().unwrap().to_ascii_lowercase();
    if is_vowel(first_char) {
        add_ay(text.to_string())
    } else {
        decal_cons(text.to_string())
    }
}

fn add_ay(s: String) -> String {
    s + "ay"
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'u' | 'o' => true,
        _ => false,
    }
}

fn decal_cons(s: String) -> String {
    let mut res = "".to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i].to_ascii_lowercase();
        if is_vowel(c) && !res.is_empty() {
            break;
        } else if
            c == 'q' &&
            i + 1 < chars.len() &&
            chars[i + 1].to_ascii_lowercase() == 'u' &&
            i != 0
        {
            res.push(chars[i]);
            res.push(chars[i + 1]);
            i += 2;
            break;
        } else {
            res.push(chars[i]);
            i += 1;
        }
    }

    let chyata: String = chars[i..].iter().collect();
    chyata + &res + "ay"
}
