use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const BASE_DIR: &str = "/sys/class/power_supply/BAT0/";
const DIVISOR: f64 = 1000000000000.0;


fn read_file(filename: &str) -> u32 {
    let filepath = PathBuf::from(BASE_DIR).join(filename);
    let mut file = File::open(filepath).expect(&format!("Could not find file {}", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&format!("Failed to read {}", filename));
    return contents.replace("\n", "").parse::<u32>().expect(&format!("Failed to parse {}", filename));
}


fn main() {
    let voltage = read_file("voltage_now");
    let current = read_file("current_now");
    let watts = (voltage as f64 * current as f64) / DIVISOR;
    println!("{:.2}W", watts);
}
