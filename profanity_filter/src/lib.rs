pub fn check_ms(message: &str) -> Result<&str, &str> {
    let sensored = String::from("stupid");
    if message.contains(&sensored) || message=="".to_string(){
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
pub struct Message;

impl Message {
    pub fn new(m1: String, _m2: String) -> String {
      m1
    }
}