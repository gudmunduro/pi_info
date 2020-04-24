use std::error::Error;
use crate::common;
use crate::common::constants;

pub fn cpu_temp() -> Result<f32, Box<dyn Error>> {
    let res = common::read_to_i32("/sys/class/thermal/thermal_zone0/temp")?;
    Ok(res as f32 / 1000_f32)
}

pub fn cpu_clock(core: i32) -> Result<f32, Box<dyn Error>> {
    let res = common::read_to_i32(&format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_cur_freq", core))?;
    Ok(res as f32 / 1000_f32)
}