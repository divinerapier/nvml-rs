fn main() {
    let driver = nvml::NVML::new().unwrap();
    let count = driver.device_get_count().unwrap();
    let version = driver.nvml_system_get_driver_version().unwrap();
    println!("count = {}", count);
    println!("version = {}", version);
}
