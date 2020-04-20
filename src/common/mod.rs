use std::error::Error;
use std::fs;

pub mod constants;

pub fn read_to_i32(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut value = fs::read_to_string(path)?;

    if value.ends_with("\\n") {
        value = String::from(&value[..(value.len() - 1)]);
    }

    Ok(value.parse::<i32>()?)
}