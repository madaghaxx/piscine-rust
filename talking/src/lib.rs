pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        "Just say something!"
    } else if text.ends_with('?') && text == text.to_uppercase() && text.chars().any(|c| c.is_alphabetic()) {
        "Quiet, I am thinking!"
    } else if text == text.to_uppercase() && text.chars().any(|c| c.is_alphabetic()) {
        "There is no need to yell, calm down!"
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
