use std::error::Error;
use crate::common;
use crate::common::constants;

pub fn cpu_temp() -> Result<f32, Box<dyn Error>> {
    let res = common::read_to_i32(constants::CPU_TEMP_PATH)?;
    Ok(res as f32 / 1000_f32)
}