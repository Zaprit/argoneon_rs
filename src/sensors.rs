use regex::Regex;
use std::{fs, process::Command};

#[derive(Debug)]
pub enum SensorError {
    HddTempError(String),
    HdParmError(String),
}

pub fn get_cpu_temp() -> Result<f32, SensorError> {
    let temp_str = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("Failed to read cpu temp");
    let temp = match temp_str.trim().parse::<f32>() {
        Ok(t) => t,
        Err(e) => panic!("Failed to parse temperature {}, source \"{}\"", e, temp_str),
    };

    Ok(temp / 1000f32)
}

const HDPARM_REGEX: &str = r"/dev/(?:[a-z0-9])+: drive state is:  ([a-z0-9/]+)$";

pub fn is_hdd_awake(path: &str) -> Result<bool, SensorError> {
    let out = Command::new("hdparm")
        .arg("-C")
        .arg(&path)
        .output()
        .expect("failed to run hdparm");

    if !out.status.success() || out.stderr.len() != 0 {
        let err_string = String::from_utf8(out.stderr).expect("hddtemp did not output utf8");
        return Err(SensorError::HdParmError(err_string));
    }

    let rx = Regex::new(HDPARM_REGEX).expect("failed to parse regex");

    let out_str = String::from_utf8(out.stdout).expect("hdparm did not output utf8");

    let trimmed_str = out_str.replace("\n", "");

    let captures = rx.captures(trimmed_str.as_str());

    if let Some(c) = captures {
        let status = c
            .get(1)
            .map(|m| m.as_str())
            .expect("idk dude, rust is confusing");

        match status {
            "standby" => return Ok(false),
            _ => return Ok(true),
        }
    } else {
        let err_string = String::from_utf8(out.stderr).expect("hddtemp did not output utf8");
        return Err(SensorError::HdParmError(err_string));
    }
}

// I swear if the hdd is more than 32767C then you've got bigger problems than integer overflow
pub fn get_hdd_temp(path: &str) -> Result<i16, SensorError> {
    let out = Command::new("hddtemp")
        .arg("-n")
        .arg("sata:".to_owned() + &path)
        .output()
        .expect("failed to run hddtemp");

    if !out.status.success() || out.stderr.len() != 0 {
        let err_string = String::from_utf8(out.stderr).expect("hddtemp did not output utf8");
        return Err(SensorError::HddTempError(err_string));
    }

    let out_str = String::from_utf8(out.stdout).expect("hddtemp did not output utf8");

    let trimmed_str = out_str.trim();

    let temp = trimmed_str
        .parse::<i16>()
        .expect("hddtemp did not output a number, and the error handling did not catch it");

    Ok(temp)
}
