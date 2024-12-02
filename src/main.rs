use std::{fs, thread::sleep, time::Duration};

mod config;
mod fanctrl;
mod files;
mod sensors;

fn main() {
    let cfg_string = fs::read_to_string("argon.toml").expect("failed to read config");

    let cfg: config::Config = toml::from_str(cfg_string.as_str()).expect("failed to parse config");
    println!("Hello, world! {:?}", cfg.fan_enabled);

    // let temp = sensors::get_cpu_temp().expect("Failed to read CPU temp");
    // println!("CPU Temp: {}", temp);

    // let hdd_temp = sensors::get_hdd_temp("/dev/sda").expect("Failed to read HDD temp");
    // println!("HDD Temp (/dev/sda): {}", hdd_temp);

    // let hdd_awake = sensors::is_hdd_awake("/dev/sdb").expect("Failed to check HDD standby status");
    // println!("Is HDD Awake: {}", hdd_awake);

    // loop {
    //     // Turn fan on
    //     fanctrl::set_fan_speed(50).expect("Failed to write fan sped");
    //     sleep(Duration::new(5, 0));

    //     // Turn fan off
    //     fanctrl::set_fan_speed(0).expect("Failed to write fan sped");
    //     sleep(Duration::new(5, 0));
    // }
}
