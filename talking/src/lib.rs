pub fn talking(text: &str) -> &str {
    let t = text.trim();
    if t.is_empty() {
        return "Just say something!";
    }
    let ques = t.ends_with('?');
    let mut alpha = false;
    let mut caps = true;

    for ch in t.chars() {
        if ch.is_alphabetic() {
            alpha = true;
            if !ch.is_uppercase() {
                caps = false;
                break;
            }
        }
    }
    let is_yelling = alpha && caps;
    if ques && is_yelling {
        "Quiet, I am thinking!"
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else if ques {
        "Sure."
    } else {
        "Interesting"
    }
}
