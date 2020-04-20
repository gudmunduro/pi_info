use std::error::Error;
use std::fs;

pub mod constants;

pub fn read_to_i32(path: &str) -> Result<i32, Box<dyn Error>> {
    let value = fs::read_to_string(path)?;

    Ok(value.trim().parse::<i32>()?)
}