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
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let Some(func) = self.flags.get(input) else {
            return Err("Flag not found".to_string());
        };
        if argv.len() >= 2 {
            func(argv[0], argv[1]).map_err(|e| e.to_string())
        } else {
            Err("Insufficient arguments".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let af = a.parse::<f64>()?;
    let bf = b.parse::<f64>()?;

    Ok((af / bf).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let af = a.parse::<f64>()?;
    let bf = b.parse::<f64>()?;

    Ok((af % bf).to_string())
}
