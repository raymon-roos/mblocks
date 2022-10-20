use std::error::Error;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Duration;

pub fn uptime() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let content = read_to_string(Path::new("/proc/uptime"))?;
    let numbers: Vec<&str> = content.split(' ').collect();
    let uptime: u64 = numbers[0].split('.').collect::<Vec<&str>>()[0].parse::<u64>()?;
    let duration = Duration::from_secs(uptime).as_secs();
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = duration / 3600;
    Ok(Box::new(format!("{}:{:02}:{:02}", hours, minutes, seconds)))
}
