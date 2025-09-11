use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: "-".to_string() + &name.chars().next().unwrap().to_string(),
            long_hand: "--".to_string() + &name.to_string(),
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
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(func) = self.flags.get(input) {
            func(argv[0], argv[1]).map_err(|e| e.to_string())
        } else {
            Err("Flag not found".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first: f32 = a.parse()?;
    let second: f32 = b.parse()?;
    Ok((first / second).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first: f32 = a.parse()?;
    let second: f32 = b.parse()?;
    Ok((first % second).to_string())
}
