use std::collections::HashMap;

// i2c address of the fan
pub const FAN_ADDR: u16 = 0x1a;

#[derive(serde::Deserialize)]
pub struct Config {
    pub hdd_fan_curve: Option<HashMap<i8, u8>>,
    pub cpu_fan_curve: Option<HashMap<i8, u8>>,

    pub fan_enabled: Option<bool>,
    pub oled_enabled: Option<bool>,
}
