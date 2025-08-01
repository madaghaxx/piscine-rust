pub fn check_ms(message: &str) -> Result<&str, &str> {
    let sensored = String::from("strupid");
    if message.to_lowercase().contains(&sensored) && message != "" {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
pub struct Message {
    message: String,
}
impl Message {
    pub fn new(mut m1: String, m2: String) -> String {
        m1 += &(" ".to_string()+&m2);
      m1
    }
}
