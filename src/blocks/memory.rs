use psutil::memory::virtual_memory;
use std::error::Error;
use std::fmt::Display;

pub fn memory_available() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let avail = virtual_memory()?.available() as f64;
    Ok(Box::new((avail * 10.0 / 1073741824.0).round() / 10.0))
}

pub fn memory_used() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let used = virtual_memory()?.used() as f64;
    Ok(Box::new((used * 10.0 / 1073741824.0).round() / 10.0))
}

pub fn memory_usage() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let vm = virtual_memory()?;
    let avail = (vm.available() as f64 * 10.0 / 1073741824.0).round() / 10.0;
    let used = (vm.used() as f64 * 10.0 / 1073741824.0).round() / 10.0;
    Ok(Box::new(format!(
        "{}|{}G",
        used.to_string(),
        avail.to_string()
    )))
}
