use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        todo!()
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        todo!()
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    todo!()
}
