pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        "Just say something!"
    } else if text.ends_with('?') && text == text.to_uppercase() {
        "Quiet, I am thinking!"
    } else if text == text.to_uppercase() {
        "There is no need to yell, calm down!"
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
