use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: & str, d: & str) -> Self {
        Flag {
            short_hand: format!("-{}",name[0..1].to_string()),
            long_hand: format!("--{}",name).to_string(),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

#[derive(Debug)]
pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }
    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(op) = self.flags.get(input) {
            let (a, b) = match (argv.get(0), argv.get(1)) {
                (Some(a), Some(b)) => (a, b),
                _ => return Err("Missing required arguments".to_string()),
            };
            
            op(a, b).map_err(|e| e.to_string())
        } else {
            Err("Error finding flag".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let aa: f64 = a.parse()?;
    let bb: f64 = b.parse()?;
    Ok(format!("{}", aa / bb))
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let aa: f64 = a.parse()?;
    let bb: f64 = b.parse()?;
    Ok(format!("{}", aa % bb))
}
