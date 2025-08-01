pub fn check_ms(message: &str) -> Result<&str, &str> {
    let sensored = String::from("strupid");
    if message.to_lowercase().contains(&sensored) {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
