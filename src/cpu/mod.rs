use std::error::Error;
use crate::common;
use crate::common::constants;

pub fn cpu_temp() -> Result<i32, Box<dyn Error>> {
    Ok(common::read_to_i32(constants::CPU_TEMP_PATH)?)
}