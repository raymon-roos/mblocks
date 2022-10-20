use crate::config;
use std::error::Error;
use std::fmt::Display;
use std::process::Command;

#[allow(dead_code)]
pub enum BlockType {
    Once,
    Interval(u64),
    Signal(i32),
}

#[allow(dead_code)]
pub enum CommandType<'a> {
    Shell(&'a [&'a str]),
    Function(fn() -> Result<Box<dyn Display>, Box<dyn Error>>),
}

pub struct Block<'a> {
    pub kind: BlockType,
    pub command: CommandType<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
}

impl Block<'_> {
    pub fn execute(&self) -> Option<String> {
        match self.command {
            CommandType::Shell(cmd) => {
                let l: usize = cmd.len();
                if l == 0 {
                    return None;
                }
                let mut command = Command::new(cmd[0]);
                if l > 1 {
                    command.args(&cmd[1..]);
                }
                let output;
                if let Ok(r) = command.output() {
                    output = r;
                } else {
                    return None;
                }
                if !output.status.success() {
                    return None;
                }
                match String::from_utf8(output.stdout) {
                    Ok(s) => Some(concat_string!(
                        self.prefix,
                        s.trim(),
                        self.suffix
                    )),
                    Err(_) => None,
                }
            }
            CommandType::Function(func) => {
                match func() {
                    Ok(r) => Some(concat_string!(self.prefix, r.to_string(), self.suffix)),
                    Err(_) => None,
                }
            }
        }
    }
}

pub fn infer_status(outputs: &[String]) -> String {
    let rootname = outputs
        .iter()
        .filter_map(|e| if !(*e).is_empty() { Some(e.to_owned()) } else { None })
        .collect::<Vec<String>>()
        .join(config::SEPARATOR);
    concat_string!(config::PREFIX, rootname, config::SUFFIX)
}
