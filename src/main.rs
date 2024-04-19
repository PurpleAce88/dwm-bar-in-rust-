const ELEMENTS:[&str;4] = ["date","pulse_volume","cpu_usage","battery"];
//options for elements are: date, pulse_volume, cpu_usage, battery, uptime, brightness
//be sure to change the array length of ELEMENTS. Array length is the number after "&str;"

use std::process::Command;
use std::{thread, time};
use sysinfo::{System, RefreshKind, CpuRefreshKind};
fn main() {
    fn get_bat() -> String{
        let bat_com = Command::new("cat")
            .arg("/sys/class/power_supply/BAT0/capacity")
            .output()
            .expect("Failed to execute battery percentage command.");
        let bat = String::from_utf8_lossy(&bat_com.stdout).to_string() + "%";
        "[Battery: ".to_owned() + &bat +"]"
    }
    fn get_time() -> String{
        let time_com = Command::new("date")
            .output()
            .expect("Failed to run date command");
        let time = "[".to_owned() + &String::from_utf8_lossy(&time_com.stdout) + "]";
        time[..23].to_string() + "]"
    }
    fn get_cpu() -> String {
        let mut s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            );
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu();
        let mut usage:f32 = 0.0;
        for cpu in s.cpus(){
            usage += cpu.cpu_usage();
        }
        usage = usage / 4.0;
        "[CPU: ".to_owned() + &usage.round().to_string() + "%]"
    }
    fn get_vol() -> String {
        let vol_com = Command::new("pamixer")
            .arg("--get-volume")
            .output()
            .expect("failed to get volume");
        let vol = "[Vol: ".to_owned() + &String::from_utf8_lossy(&vol_com.stdout).to_string() + "%]";
        vol
    }
    fn get_uptime() -> String {
        let uptime_com = Command::new("uptime")
            .arg("-p")
            .output()
            .expect("failed to get uptime");
        let mut uptime = String::from_utf8_lossy(&uptime_com.stdout).to_string();
        uptime = "[Uptime: ".to_owned() + &uptime[2..] + "]";
        uptime.to_string()
    }
    fn get_brightness() -> String {
        let brightness_com = Command::new("brightnessctl")
            .arg("g")
            .output()
            .expect("failed to get brightness");
        let brightness = String::from_utf8_lossy(&brightness_com.stdout).to_string();
        "[Brightness: ".to_owned() + &brightness + "]"
    }
    loop{
        let mut bar_elements = " ".to_string();
        for x in ELEMENTS {
            if x == "battery" {
                bar_elements = bar_elements.to_owned() + &get_bat();
            }if x == "date" {
                bar_elements = bar_elements.to_owned() + &get_time();
            }if x == "cpu_usage" {
                bar_elements = bar_elements.to_owned() + &get_cpu();
            }if x == "pulse_volume" {
                bar_elements = bar_elements.to_owned() + &get_vol();
            }if x == "uptime" {
                bar_elements = bar_elements.to_owned() + &get_uptime();
            }if x == "brightness" {
                bar_elements = bar_elements.to_owned() + &get_brightness();
            }
        }
        let bar = bar_elements.clone();
        let _set = Command::new("xsetroot")
            .arg("-name")
            .arg(bar.clone())
            .status()
            .expect("Failed to set bar");
        thread::sleep(time::Duration::from_secs(1));
    }
}
